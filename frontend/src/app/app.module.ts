import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';

import { HttpClientModule } from '@angular/common/http';


import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RequestComponent } from './request/request.component';
import { AnswerComponent } from './answer/answer.component';

import { SbbModule } from './sbb.module';
import { RequestAnswerComponentComponent } from './request-answer-component/request-answer-component.component';
import { MeteoComponent } from './meteo/meteo.component';
import { TrainsComponent } from './trains/trains.component';
import { StationsComponent } from './stations/stations.component';@NgModule({
  declarations: [
    AppComponent,
    RequestComponent,
    AnswerComponent,
    RequestAnswerComponentComponent,
    MeteoComponent,
    TrainsComponent,
    StationsComponent,
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

