import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-meteo',
  templateUrl: './meteo.component.html',
  styleUrls: ['./meteo.component.scss']
})
export class MeteoComponent implements OnInit {

  constructor() { }

  ngOnInit(): void {
  }

  meteo :string = "It's going to be sunny in";
  meteoIcon = "kom:sunshine-medium";
  location = "Zurich";

}
