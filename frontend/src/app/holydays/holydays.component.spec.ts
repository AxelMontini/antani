import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HolydaysComponent } from './holydays.component';

describe('HolydaysComponent', () => {
  let component: HolydaysComponent;
  let fixture: ComponentFixture<HolydaysComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ HolydaysComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(HolydaysComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
