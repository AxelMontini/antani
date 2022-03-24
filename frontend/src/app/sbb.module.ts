import { NgModule } from '@angular/core';
import { SbbButtonModule } from '@sbb-esta/angular/button';
import { SbbCheckboxModule } from '@sbb-esta/angular/checkbox';
import { SbbTimeInputModule } from '@sbb-esta/angular/time-input'; 
import { SbbDatepickerModule } from '@sbb-esta/angular/datepicker'; 
import { SbbFormFieldModule } from '@sbb-esta/angular/form-field'; 
import { SbbInputModule } from '@sbb-esta/angular/input'; 
import {SbbIconModule} from '@sbb-esta/angular/icon';

const modules = [
    SbbIconModule,
    SbbButtonModule,
    SbbCheckboxModule,
    SbbTimeInputModule,
	  SbbDatepickerModule,
	  SbbFormFieldModule,
	  SbbInputModule
]

@NgModule({
  imports: modules,
  exports: modules,
})

export class SbbModule {}
