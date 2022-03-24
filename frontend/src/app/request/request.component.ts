import { Component, OnInit } from '@angular/core';

import { FormsModule, FormControl, Validators, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-request',
  templateUrl: './request.component.html',
  styleUrls: ['./request.component.scss']
})
export class RequestComponent implements OnInit {
  date: FormControl = new FormControl(new Date(), [Validators.required]);

  inputFormControl = new FormControl();
  constructor() { }

  ngOnInit(): void {
  }

}

