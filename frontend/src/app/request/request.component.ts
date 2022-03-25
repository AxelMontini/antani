import { Component, OnInit, Output, EventEmitter } from '@angular/core';

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

export type Message = {
  showLoading: boolean,
  from: string,
  to:string,
  dateTimeDep: string,
  dateTimeArr: string,
  lastYearDate: string
}

@Component({
  selector: 'app-request',
  templateUrl: './request.component.html',
  styleUrls: ['./request.component.scss'],
})
export class RequestComponent implements OnInit {
  date: FormControl = new FormControl(new Date(), [
    Validators.required,
  ]);
  dep: FormControl = new FormControl('', [
    Validators.required,
    this.validateTimeFormat(),
    this.validateTime(() => this.date.value),
  ]);
  ret: FormControl = new FormControl('', [
    Validators.required,
    this.validateTimeFormat(),
    this.validateTime(() => this.depDatetime),
  ]);

  tryingSend = false;

  // From and to stations helpful stuff
  from: FormControl = new FormControl('', [
    Validators.required,
    this.validateStation,
  ]);
  to: FormControl = new FormControl('', [
    Validators.required,
    this.validateStation,
  ]);

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

  validateTimeFormat(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      let time = control.value.split(':');
      if (time.length != 2) return { invalid: true };
      let hour = parseInt(time[0]);
      if (isNaN(hour) || hour < 0 || hour > 23) return { invalid: true };
      let minute = parseInt(time[1]);
      if (isNaN(minute) || minute < 0 || minute > 59) return { invalid: true };
      return null;
    };
  }

  validateStation(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      let statFound: string[] = [];
      this.loadStations(control.value, statFound);
      if (statFound.length == 0) {
        return { valid: false };
      } else {
        return null;
      }
    };
  }

  optionsFrom: string[] = this.defaultOptions;
  optionsTo: string[] = this.defaultOptions;

  ngOnInit() {}

  autoCompleteFrom() {
    this.loadStations(this.from.value, this.optionsFrom);
    if (this.optionsFrom.length == 0) this.optionsFrom = this.defaultOptions;
  }

  autoCompleteTo() {
    this.loadStations(this.to.value, this.optionsTo);
    if (this.optionsTo.length == 0) this.optionsTo = this.defaultOptions;
  }

  loadStations(str: string, stationFound: string[]) {
    if (str != '')
      fetch(`/api/stations/${str}`)
        .then((r) => r.json())
        .then((r) => {
          stationFound = r['stations'];
        });
  }

  // End

  date_past: boolean = false;

  minDate = new Date();

  inputFormControl = new FormControl();
  constructor() {}

  /* Send info so that answer component knows that request has been submitted */

  @Output() messageEvent = new EventEmitter<Message>();
  /*   End of sending */

  handleClick() {
    let toTemp = this.to.value;
    this.to.setValue(this.from.value);
    this.from.setValue(toTemp);
  }


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
    console.log(this.date.value);
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
        dateTimeDep: this.depDatetime.toISOString(),
        dateTimeArr: this.retDatetime.toISOString(),
        lastYearDate: (new Date(+this.depDatetime-31536000000)).toISOString(),
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
