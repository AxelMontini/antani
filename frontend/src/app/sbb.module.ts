import { NgModule } from '@angular/core';
import { SbbButtonModule } from '@sbb-esta/angular/button';
import { SbbCheckboxModule } from '@sbb-esta/angular/checkbox';
import { SbbTimeInputModule } from '@sbb-esta/angular/time-input'; 
import { SbbDatepickerModule } from '@sbb-esta/angular/datepicker'; 
import { SbbFormFieldModule } from '@sbb-esta/angular/form-field'; 
import { SbbInputModule } from '@sbb-esta/angular/input'; 
import {SbbIconModule} from '@sbb-esta/angular/icon';
import {SbbLoadingModule} from '@sbb-esta/angular/loading';
import {SbbAccordionModule} from '@sbb-esta/angular/accordion'; 
import {SbbNotificationModule} from '@sbb-esta/angular/notification';

const modules = [
  SbbNotificationModule,
    SbbLoadingModule,
    SbbIconModule,
    SbbButtonModule,
    SbbCheckboxModule,
    SbbTimeInputModule,
	  SbbDatepickerModule,
	  SbbFormFieldModule,
	  SbbInputModule,
	  SbbAccordionModule,
]

@NgModule({
  imports: modules,
  exports: modules,
})

export class SbbModule {}
