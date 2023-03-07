import React from "react";
import gameManager from "../index.js";
import { create_gameState } from "../game/gameState";
import {Main} from "../Main";
import { JoinGameMenu } from "./JoinGameMenu";

export class StartMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }

    render(){return(<div>
        <button style={{width: "90%"}} onClick={()=>{
            gameManager.gameState = create_gameState();
            Main.instance.setState({panels : [<JoinGameMenu/>]});
        }}>Play</button><br/>
        <br/>
        <button style={{width: "90%"}}>Login</button><br/>
    </div>)}
}