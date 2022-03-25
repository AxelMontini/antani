import { Component, OnInit, Input, SimpleChanges } from '@angular/core';
import { Message } from '../request/request.component';

@Component({
  selector: 'app-meteo',
  templateUrl: './meteo.component.html',
  styleUrls: ['./meteo.component.scss']
})
export class MeteoComponent implements OnInit {

  @Input() message: Message | undefined;

  constructor() { }

  ngOnInit(): void {
    
  }

  precipitation = "";
  meteoIcon = "kom:cloud-large";
  temperature = '';
  location: string | undefined;
  isHolyday = true;
  date  :string | undefined = '';
  rain = '';

  meteoIcons: any = [
    "kom:question-mark-medium",
    "kom:sunshine-large",
    "kom:cloud-sunshine-medium",
    "kom:cloud-medium",
    "kom:cloud-drops-medium",
    "kom:cloud-rain-snow-medium",    
    "kom:cloud-snow-medium",
    "kom:cloud-strong-rain-sun-medium",
    "kom:cloud-little-snow-sun-medium",
    "kom:cloud-rain-snow-medium",
    "kom:cloud-fog-small",
    "kom:cloud-dense-fog-small",  
    "kom:cloud-rain-snow-medium",,
    "kom:cloud-lightning-medium",
    "kom:cloud-rain-medium"
  ];


  ngOnChanges(changes: SimpleChanges) {
    fetch(`/api/weather?date=${this.message?.dateTimeArr}&station=${this.message?.to}`)
      .then(r => r.json())
      .then(r => {
        this.meteoIcon = this.meteoIcons[r.data[2].value];
        this.temperature = r.data[0].value;
        this.precipitation = r.data[1].value;
        this.location = this.message?.to;
        if (parseFloat(this.precipitation) > 0.1) {
          this.rain = " and it will be raining, be ready!"
        }
        else {
          this.rain = " and the sun will shine, so trains might be more crowded than usual!"
        }
      })
    
  }

}
