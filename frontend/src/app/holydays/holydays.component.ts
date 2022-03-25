import { Component, OnInit, Input, SimpleChanges } from '@angular/core';
import { Message } from '../request/request.component';

@Component({
  selector: 'app-holydays',
  templateUrl: './holydays.component.html',
  styleUrls: ['./holydays.component.scss']
})
export class HolydaysComponent implements OnInit {
  @Input() message: Message | undefined;
  constructor() { }
  
  location :string | undefined;

  ngOnInit(): void {
  }

  ngOnChanges(changes :SimpleChanges) {
    
    this.location = this.message?.to;
  }
}
