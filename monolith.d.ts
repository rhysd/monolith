/* tslint:disable */
/**
* @param {string} url_target 
* @param {MonolithOptions} options 
* @returns {any} 
*/
export function monolithOfUrl(url_target: string, options: MonolithOptions): any;
/**
* @param {string} html 
* @param {string} final_url 
* @param {MonolithOptions} options 
* @returns {any} 
*/
export function monolithOfHtml(html: string, final_url: string, options: MonolithOptions): any;
/**
*/
export class MonolithOptions {
  free(): void;
/**
* @returns {MonolithOptions} 
*/
  static new(): MonolithOptions;
/**
* @param {boolean} b 
*/
  noCss(b: boolean): void;
/**
* @param {boolean} b 
*/
  noFrames(b: boolean): void;
/**
* @param {boolean} b 
*/
  noImages(b: boolean): void;
/**
* @param {boolean} b 
*/
  noJs(b: boolean): void;
/**
* @param {boolean} b 
*/
  isolate(b: boolean): void;
/**
* @param {boolean} b 
*/
  silent(b: boolean): void;
/**
* @param {string} ua 
*/
  userAgent(ua: string): void;
}
