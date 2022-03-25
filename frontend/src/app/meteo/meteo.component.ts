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


  meteo :string = "It's going to be sunny in";
  meteoIcon = "kom:sunshine-medium";
  location: string | undefined;
  isHolyday = true;

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
    this.location = this.message?.to;
  }

}
