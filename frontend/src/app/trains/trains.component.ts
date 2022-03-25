import { Component, OnInit, Input, SimpleChanges } from '@angular/core';
import { SbbTableDataSource } from '@sbb-esta/angular/table';
import * as Train from './train_interface';


interface Connection {
  name: string;
  description: string;
  icon?: string;
  occupation: "none" | "low" | "medium" | "high";
}

interface Type {
  departureStation: string,
  arrivalStation: string,
  departureTime: string,
  arrivalTime: string,
  duration:string,
}

@Component({
  selector: 'app-trains',
  templateUrl: './trains.component.html',
  styleUrls: ['./trains.component.scss']
})

export class TrainsComponent implements OnInit {
  @Input() connections: any;
  constructor() { 


  }

  occupancy: string[] = [
    "none",
    "low",
    "medium",
    "high"
  ];

  ngOnInit(): void {

  }

  ngOnChanges(changes: SimpleChanges): void {
    console.log(this.connections);
    console.log(this.connections[0].connection);
  }

  displayedColumns: string[] = [
    'columnOne',
    'columnTwo',
    'columnThree',
    'columnFour',
    'columnFive',
  ];
  dataSource: SbbTableDataSource<any> = new SbbTableDataSource(TABLE_EXAMPLE_DATA_SIMPLE);

  // tabulate(c :connection): SbbTableDataSource<any> {
  //   let tableContent: Row = [];
  //   row = {

  //   }

  //   return
  // }
}

const TABLE_EXAMPLE_DATA_SIMPLE = [
  {
    columnOne: 'columnOne',
    columnTwo: 'columnTwo',
    columnThree: 'columnThree',
    columnFour: 'columnFour',
    columnFive: 'columnFive',
  },
  {
    columnOne: 'a',
    columnTwo: 'b',
    columnThree: 'c',
    columnFour: 'd',
    columnFive: 'e',
  },
];
