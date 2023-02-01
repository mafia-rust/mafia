import React from "react";
import namesJson from "../names"

export class JoinGameMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            nameValue: "",
            roomCodeValue: "",
        };
    }
    componentDidMount() {
        //generate random name
        this.setState({nameValue: 
            namesJson.defaultNames[
                Math.floor(
                    Math.random()*namesJson.defaultNames.length
                )
            ]
        });
    }
    componentWillUnmount() {
    }

    render(){return(<div>
        Room Code<br/>
        <input type="text" value={this.state.roomCodeValue} 
            onChange={(e)=>{this.setState({roomCodeValue: e.target.value})}}
        /><br/>
        <br/>
        Name<br/>
        <input type="text" value={this.state.nameValue} 
            onChange={(e)=>{this.setState({nameValue: e.target.value})}}
        /><br/>
        <br/>
        <br/>
        <button style={{width: "90%"}}>Join Lobby</button><br/>
        <br/>
        <button style={{width: "90%"}}>New Lobby</button><br/>
    </div>)}
}