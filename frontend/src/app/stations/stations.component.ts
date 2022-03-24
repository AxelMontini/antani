import { FormControl, FormControlName } from '@angular/forms';
import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'app-stations',
  templateUrl: './stations.component.html',
  styleUrls: ['./stations.component.scss']
})
export class StationsComponent implements OnInit {
  @Input() label:string ='';
  @Input() control:FormControl = new FormControl();

  default_options: string[] = [
    'Zurich',
    'Bern',
    'Basel',
    'Lausanne',
    'Luzern',
    'St. Gallen',
    'Lugano',
    'Thun',
  ];

  options: string[] = this.default_options;

  constructor() { }

  ngOnInit(): void {
  }

}
