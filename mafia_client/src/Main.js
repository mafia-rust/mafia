import React from "react";
import gameManager from "./gameManager";

export class Main extends React.Component {
    static instance;
    constructor(props) {
        super(props);

        this.state = {

        };
    }
    componentDidMount() {
        Main.instance = this;
    }
    componentWillUnmount() {
        Main.instance = undefined;
    }
    render(){return(<div>
        <br/>
        <br/>
        <br/>
        <br/>
        Funny monkey

        <br/>
        <br/>
        <br/>
        <br/>
    </div>)}
}

export default Main;