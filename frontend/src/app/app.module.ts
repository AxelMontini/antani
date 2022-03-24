import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';

import { HttpClientModule } from '@angular/common/http';


import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { DescriptionComponent } from './description/description.component';
import { RequestComponent } from './request/request.component';
import { AnswerComponent } from './answer/answer.component';

import { SbbModule } from './sbb.module';
import { RequestAnswerComponentComponent } from './request-answer-component/request-answer-component.component';
import { MeteoComponent } from './meteo/meteo.component';
import { TrainsComponent } from './trains/trains.component';@NgModule({
  declarations: [
    AppComponent,
    DescriptionComponent,
    RequestComponent,
    AnswerComponent,
    RequestAnswerComponentComponent,
    MeteoComponent,
    TrainsComponent,
  ],
  imports: [
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule,
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    SbbModule,
    ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }

export class TrainChooChooAppModule {}

