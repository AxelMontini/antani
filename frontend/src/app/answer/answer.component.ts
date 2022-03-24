import { Component, OnInit, Input, OnChanges, SimpleChanges } from '@angular/core';

@Component({
  selector: 'app-answer',
  templateUrl: './answer.component.html',
  styleUrls: ['./answer.component.scss']
})

export class AnswerComponent implements OnInit, OnChanges {
  


  constructor() { }

  ngOnInit(): void {
  }

  /* Request handling */
  @Input() showLoading: boolean = false;
  @Input() showMeteo: boolean = false;
  @Input() showTrains: boolean = false;
  /* */

  ngOnChanges(changes: SimpleChanges) {
    if(this.showLoading) {
      this.showMeteo = true;
      this.showTrains = true;
      this.showLoading = false;
    }
  }
}
