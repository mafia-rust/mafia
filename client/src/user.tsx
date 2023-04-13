
export class UserData {
    /*
    Temporary class for user data. Inner data should be replaced with account stuff RE: auth0
    */
    private name: string;

    constructor(name: string) {
        this.name = name;
    }

    getAccountName(): string {
        return this.name;
    }
}