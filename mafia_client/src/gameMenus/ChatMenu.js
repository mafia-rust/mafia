import React from "react";
import { getChatString } from "../game/lang.js";
import gameManager from "../index";

export class ChatMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            chatFeild: "",
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            });
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    //helper functions
    sendChatFeild() {
        let text = this.state.chatFeild.trim();
        if(text.startsWith("/w")){
            try{
                let playerIndex = Number(text[2])-1;
                text = text.substring(3);
                gameManager.sendWhisper_button(playerIndex, text);
            }catch(e){
                gameManager.sendMessage_button(text);
            }
        }else{
            gameManager.sendMessage_button(text);
        }
        this.setState({chatFeild:""});
    }

    //these next functions are old
    componentDidUpdate() {
        if(this.bottomIsInViewport(500))   //used to be 500
            this.scrollToBottom();
    }
    scrollToBottom() {
        this.buttomOfPage.scrollIntoView({ behavior: "smooth" });
    }
    bottomIsInViewport(offset = 0) {
        if (!this.buttomOfPage) return false;
        const top = this.buttomOfPage.getBoundingClientRect().top;
        //if top is between 0 and height then true
        //else false
        return (top + offset) >= 0 && (top - offset) <= window.innerHeight;
    }

    renderTextInput(){return(<div style={{position: "sticky", bottom: 10}}>
        <input value={this.state.chatFeild} onChange={(e)=>{this.setState({chatFeild: e.target.value.trimStart()})}} 
            onKeyPress={(e) => {
            if(e.code === "Enter") {
                this.sendChatFeild();
            }
        }}></input>
        <button onClick={()=>{
            this.sendChatFeild();
        }}>Send</button>
    </div>);}
    renderChatMessage(msg, i) {
        return(<div key={i} style={{textAlign:"left"}}>
            {getChatString(msg)}
        </div>);
    }
    render(){return(<div>
        {this.state.gameState.chatMessages.map((msg, i)=>{
            return this.renderChatMessage(msg, i);
        }, this)}

        <br ref={(el) => { this.buttomOfPage = el; }}/>
            {this.renderTextInput()}
        <br/>
        <br/>
        <br/>
    </div>)}
}