export class HttpException extends Error {
  constructor(message, public payload: any) {
    super(message);
  }
}
