import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

import {
  FormsModule,
  FormControl,
  Validators,
  ReactiveFormsModule,
} from '@angular/forms';
import { AnswerComponent } from '../answer/answer.component';

export type Message = {
  showLoading: boolean,
  from: string,
  to:string,
  dateTimeDep: string
  dateTimeArr: string
}

@Component({
  selector: 'app-request',
  templateUrl: './request.component.html',
  styleUrls: ['./request.component.scss'],
})
export class RequestComponent implements OnInit {
  date: FormControl = new FormControl(new Date(), [Validators.required]);
  ret: FormControl = new FormControl('', [Validators.required]);
  dep: FormControl = new FormControl('', [Validators.required]);

  // From and to stations helpful stuff
  from: FormControl = new FormControl('', [Validators.required]);
  to: FormControl = new FormControl('', [Validators.required]);

  defaultOptions: string[] = [
    'Zurich',
    'Bern',
    'Basel',
    'Lausanne',
    'Luzern',
    'St. Gallen',
    'Lugano',
    'Thun',
  ];

  optionsFrom: string[] = this.defaultOptions;
  optionsTo: string[] = this.defaultOptions;

  ngOnInit() {}

  autoCompleteFrom() {
    if (this.from.value != '')
      fetch(`/api/stations/${this.from.value}`)
        .then((r) => r.json())
        .then((r) => (this.optionsFrom = r['stations']));
    else this.optionsFrom = this.defaultOptions;
  }

  autoCompleteTo() {
    if (this.to.value != '')
      fetch(`/api/stations/${this.to.value}`)
        .then((r) => r.json())
        .then((r) => (this.optionsTo = r['stations']));
    else this.optionsTo = this.defaultOptions;
  }

  // End

  date_past: boolean = false;

  minDate = new Date();

  inputFormControl = new FormControl();
  constructor() {}

  /* Send info so that answer component knows that request has been submitted */
  
  @Output() messageEvent = new EventEmitter<Message>();
  /*   End of sending */

  get dep_time(): Time {
    return new Time(this.dep.value);
  }

  get ret_time(): Time {
    return new Time(this.ret.value);
  }

  get time_invalid(): boolean {
    return this.dep_time.invalid || this.ret_time.invalid;
  }

  get dep_datetime(): DateTime {
    return new DateTime(this.date.value, this.dep_time, new Date());
  }

  get ret_datetime(): DateTime {
    return new DateTime(this.date.value, this.ret_time, this.dep_datetime);
  }

  submit(): void {
    if(this.dep_time.valid && this.ret_time.valid && this.from.valid && this.to.valid && this.dep_datetime.valid && this.ret_datetime.valid) {
      /* Send to answer module */
      let message: Message = {
        showLoading: true,
        from: this.from.value,
        to: this.to.value,
        dateTimeDep: this.dep_datetime.toISOString(),
        dateTimeArr: this.ret_datetime.toISOString(),
      };
      this.messageEvent.emit(message);
      /* */
    }
  }
}

class Time {
  hour_invalid: boolean = false;
  minute_invalid: boolean = false;
  invalid: boolean = false;
  empty: boolean;

  hour: number;
  minute: number;

  constructor(time_str: string) {
    let time = time_str.split(':');
    this.hour = parseInt(time[0]);
    if (isNaN(this.hour) || this.hour < 0 || this.hour > 23)
      this.hour_invalid = true;
    this.minute = parseInt(time[1]);
    if (isNaN(this.minute) || this.minute < 0 || this.minute > 59)
      this.minute_invalid = true;
    this.invalid = (this.hour_invalid || this.minute_invalid) && time_str != '';
    this.empty = time_str=='';
  } 

  get valid(): boolean {
    return !this.empty && !this.invalid;
  }
}

class DateTime extends Date {
  past: boolean;
  constructor(date: Date, time: Time, ref: Date) {
    super(date);
    this.setHours(time.hour, time.minute, 0);
    this.past = +this < +ref;
  }

  get valid(): boolean {
    return !this.past;
  }
}
