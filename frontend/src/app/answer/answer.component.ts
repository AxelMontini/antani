import { Component, OnInit, Input } from '@angular/core';

@Component({
  selector: 'app-answer',
  templateUrl: './answer.component.html',
  styleUrls: ['./answer.component.scss']
})

export class AnswerComponent implements OnInit {
  


  constructor() { }

  ngOnInit(): void {
  }

  /* Request handling */
  @Input() showLoading: boolean = false;
  /* */
  showMeteo :boolean = true;
  showTrains :boolean = true;

}
