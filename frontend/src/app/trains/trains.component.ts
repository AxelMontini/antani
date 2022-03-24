import { Component, OnInit } from '@angular/core';


interface Connection {
  name: string;
  description: string;
}

@Component({
  selector: 'app-trains',
  templateUrl: './trains.component.html',
  styleUrls: ['./trains.component.scss']
})
export class TrainsComponent implements OnInit {
  trains: Connection[] = [
    {
      name: "lol",
      description: "Ciufciuf"
    },
    {
      name: "1",
      description: "Ciufciuf1"
    },
    {
      name: "2",
      description: "Ciufciuf2"
    },
  ]

  constructor() { 


  }

  ngOnInit(): void {
  }

}
