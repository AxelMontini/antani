import { Component, OnInit, Input, SimpleChanges } from '@angular/core';
import { SbbTableDataSource } from '@sbb-esta/angular/table';
import { Connection, Prognosis } from './train_interface';
import * as Train from './train_interface';



interface Row {
  product: string,
  departureStation: string,
  arrivalStation: string,
  departureTime?: string | null,
  arrivalTime?: string | null,
  departurePlatform?: string | null | unknown,
  arrivalPlatform?: string | null | unknown,
  duration?:string | null,
  occupancy1stIcon?: string | null,
  occupancy2ndIcon?: string | null
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
    'product',
    'departureStation',
    'arrivalStation',
    'departureTime',
    'arrivalTime',
    'departurePlatform',
    'arrivalPlatform',
    'duration',
    'occupancy',
  ];

  getTime(str?: string | null): string {
    if(str) {
      let times = str.split('T')[1].split(':');
      return `${times[0]}:${times[1]}`
    }
    else return '';
  }

  getDur(start?: string | null, end?: string | null): string {
    if(start && end) {
      let s = Date.parse(start);
      let e = Date.parse(end);
      let dt = new Date(e-s);
      return this.getTime(dt.toISOString());
    }
    return '';
  }

  tabulate(c: Connection): SbbTableDataSource<any> {

    console.log(c);
    let tableContent: Row[] = [];


    c.sections.map((s: Train.Section,i) => {

      tableContent.push({
        product: c.products[i],
        departureStation: s.departure.station.name,
        arrivalStation: s.arrival.station.name,
        departureTime: this.getTime(s.departure.departure),
        arrivalTime: this.getTime(s.arrival.arrival),
        departurePlatform: s.departure.station['platform'] || s.departure.prognosis?.['platform'],
        arrivalPlatform: s.arrival.station['platform'] || s.arrival.prognosis?.['platform'],
        duration: this.getDur(s.departure.departure,s.arrival.arrival),
        occupancy1stIcon: "fpl:utilization-low",
        occupancy2ndIcon: "fpl:utilization-medium"
      })
    })

    console.log(tableContent);


    return new SbbTableDataSource(tableContent);
  }
}
