import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

import {
  FormsModule,
  FormControl,
  Validators,
  ReactiveFormsModule,
  Validator,
  ValidatorFn,
  ValidationErrors,
  AbstractControl,
} from '@angular/forms';
import { AnswerComponent } from '../answer/answer.component';

export type Message = {
  showLoading: boolean;
  from: string;
  to: string;
  depDatetime: string;
  retDatetime: string;
};

@Component({
  selector: 'app-request',
  templateUrl: './request.component.html',
  styleUrls: ['./request.component.scss'],
})
export class RequestComponent implements OnInit {
  date: FormControl = new FormControl(new Date(), [Validators.required]);
  dep: FormControl = new FormControl('', [
    Validators.required,
    this.validateTime(() => this.date.value),
  ]);
  ret: FormControl = new FormControl('', [
    Validators.required,
    this.validateTime(() => this.depDatetime),
  ]);

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

  validateTime(dateGetter: () => Date): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      const forbidden =
        +new DateTime(this.date.value, new Time(control.value)) <=
        +dateGetter();
      return forbidden ? { past: true } : null;
    };
  }



  validateStation(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      
    }
  }

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
    
    
  }

  loadStations(str: string) {
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

  get depTime(): Time {
    return new Time(this.dep.value);
  }

  get retTime(): Time {
    return new Time(this.ret.value);
  }

  get depDatetime(): DateTime {
    return new DateTime(this.date.value, this.depTime);
  }

  get retDatetime(): DateTime {
    return new DateTime(this.date.value, this.retTime);
  }

  submit(): void {
    if (
      this.depTime.valid &&
      this.retTime.valid &&
      this.from.valid &&
      this.to.valid
    ) {
      /* Send to answer module */
      let message: Message = {
        showLoading: true,
        from: this.from.value,
        to: this.to.value,
        depDatetime: this.depDatetime.toISOString(),
        retDatetime: this.retDatetime.toISOString(),
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

  valid: boolean;

  constructor(time_str: string) {
    let time = time_str.split(':');
    this.hour = parseInt(time[0]);
    if (isNaN(this.hour) || this.hour < 0 || this.hour > 23)
      this.hour_invalid = true;
    this.minute = parseInt(time[1]);
    if (isNaN(this.minute) || this.minute < 0 || this.minute > 59)
      this.minute_invalid = true;
    this.invalid = (this.hour_invalid || this.minute_invalid) && time_str != '';
    this.empty = time_str == '';
    this.valid = !this.empty && !this.invalid;
  }
}

class DateTime extends Date {
  constructor(date: Date, time: Time) {
    super(date);
    this.setHours(time.hour, time.minute, 0);
  }
}
