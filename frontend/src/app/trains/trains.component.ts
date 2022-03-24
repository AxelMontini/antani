import { Component, OnInit } from '@angular/core';


interface Connection {
  name: string;
  description: string;
  icon?: string;
  occupation: "none" | "low" | "medium" | "high";
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
      description: "Ciufciuf",
      icon: "kom:train",
      occupation: "low"
    },
    {
      name: "1",
      description: "Ciufciuf1",
      occupation:"low"
    },
    {
      name: "2",
      description: "Ciufciuf2",
      occupation:"none"
    },
  ]

  constructor() { 


  }

  ngOnInit(): void {
  }

}
