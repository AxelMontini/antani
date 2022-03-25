/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

 export type LocationType = "station" | "poi" | "address" | "refine";

 export interface ConnectionResponse {
   connections: Connection[];
   [k: string]: unknown;
 }
 export interface Connection {
   capacity1st?: number | null;
   capacity2nd?: number | null;
   duration: string;
   from: Checkpoint;
   products: string[];
   sections: Section[];
   service?: Service | null;
   to: Checkpoint;
   [k: string]: unknown;
 }
 export interface Checkpoint {
   arrival?: string | null;
   delay?: number | null;
   departure?: string | null;
   platform?: string | null;
   prognosis?: Prognosis | null;
   station: Location;
   [k: string]: unknown;
 }
 export interface Prognosis {
   arrival?: string | null;
   capacity1st?: string | null;
   capacity2nd?: string | null;
   departure?: string | null;
   platform?: string | null;
   [k: string]: unknown;
 }
 export interface Location {
   coordinate: Coordinate;
   distance?: number | null;
   id: string;
   name: string;
   score?: number | null;
   type?: LocationType | null;
   [k: string]: unknown;
 }
 export interface Coordinate {
   type: string;
   /**
	* latitude
	*/
   x: number;
   /**
	* longitude
	*/
   y: number;
   [k: string]: unknown;
 }
 export interface Section {
   arrival: Checkpoint;
   departure: Checkpoint;
   journey?: Journey | null;
   walk?: number | null;
   [k: string]: unknown;
 }
 export interface Journey {
   capacity1st?: number | null;
   capacity2nd?: number | null;
   category: string;
   categoryCode?: number | null;
   name: string;
   number: string;
   operator: string;
   passList: Checkpoint[];
   to: string;
   [k: string]: unknown;
 }
 /**
  * What is this for???
  */
 export interface Service {
   irregular: string;
   regular: string;
   [k: string]: unknown;
 }