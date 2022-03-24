import { Component, OnInit, Output, EventEmitter } from '@angular/core';

import { FormsModule, FormControl, Validators, ReactiveFormsModule } from '@angular/forms';
import { AnswerComponent } from '../answer/answer.component';

@Component({
  selector: 'app-request',
  templateUrl: './request.component.html',
  styleUrls: ['./request.component.scss']
})
export class RequestComponent implements OnInit {
  date: FormControl = new FormControl(new Date(), [Validators.required]);
  ret: FormControl = new FormControl('', [Validators.required]);
  dep: FormControl = new FormControl('', [Validators.required]);

  // From and to stations helpful stuff
  from: FormControl = new FormControl('', [Validators.required]);
  to: FormControl = new FormControl('', [Validators.required]);

  searchControl = new FormControl('');
  searchValues: string[] = [];
  options: string[] = [
    'Zurich',
    'Bern',
    'Basel',
    'Lausanne',
    'Luzern',
    'St. Gallen',
    'Lugano',
    'Thun',
  ];

  handleSearch(value: string) {
    // Only display up to the last five search values.
    // This is only for the purpose of this example.
    this.searchValues = [value, ...this.options].slice(0, 5);
  }

  // End

  date_past: boolean = false;

  minDate = new Date();

  inputFormControl = new FormControl();
  constructor() { }

  /* Send info so that answer component knows that request has been submitted */
  message: boolean = true;

  @Output() messageEvent = new EventEmitter<boolean>();

  sendMessage() {
    this.messageEvent.emit(this.message)
  }
  /*   End of sending */

  ngOnInit(): void {

  }

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
    if(!this.time_invalid) {
      
      console.log(this.dep_datetime);
      /* Send to answer module */
      this.sendMessage()
      /* */
    }

  }
}

class Time {

  hour_invalid: boolean = false;
  minute_invalid: boolean = false;
  invalid: boolean = false;

  hour: number;
  minute: number;

  constructor(time_str :string) {
    let time = time_str.split(":");
    this.hour = parseInt(time[0]);
    if(isNaN(this.hour) || this.hour<0 || this.hour>23) this.hour_invalid = true;
    this.minute = parseInt(time[1]);
    if(isNaN(this.minute) || this.minute<0 || this.minute>59) this.minute_invalid = true;
    this.invalid = (this.hour_invalid || this.minute_invalid) && time_str!="";
  }
}

class DateTime extends Date {
  past: boolean;
  constructor(date: Date, time: Time, ref: Date) {
    super(date)
    this.setHours(time.hour,time.minute,0);
    this.past = +this < +ref;
  }
}

