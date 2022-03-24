import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RequestAnswerComponentComponent } from './request-answer-component.component';

describe('RequestAnswerComponentComponent', () => {
  let component: RequestAnswerComponentComponent;
  let fixture: ComponentFixture<RequestAnswerComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RequestAnswerComponentComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(RequestAnswerComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
