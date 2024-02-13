import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css"
import "./startMenu.css"
import translate from "../../game/lang";
// import WikiSearch from "../../components/WikiSearch";
import Anchor from "../Anchor";
// import * as LoadingScreen from "../LoadingScreen";
import LoadingScreen from "../LoadingScreen";
import StandaloneWiki from "./StandaloneWiki";

type StartMenuProps = {
}
type StartMenuState = {
} 

export default class StartMenu extends React.Component<StartMenuProps, StartMenuState> {

    private async connectButton() {
        Anchor.setContent(<LoadingScreen type="default"/>);

        GAME_MANAGER.setOutsideLobbyState();
    }
    //Temp func for audio testing
    private async importantButton(){
        Anchor.playAudioFile("/audio/longSpeech.mp4",false);
    }

    render(){return(<div className="sm">
        <main>
            <section id="main">
                <h1>{translate("menu.start.title")}</h1>
                
                <div>
                    <button onClick={()=>{this.connectButton()}}>
                        {translate("menu.start.button.play")}
                    </button>
                </div>
                {/*
                test div for audio
                */}
                <div>
                    <button onClick={()=>{this.importantButton()}}>
                    </button>
                </div>
            </section>
            {/* <section id="wiki">
                <h2>{translate("menu.wiki.title")}</h2>
                <WikiSearch/>
            </section> */}
        </main>
        <footer>
            <nav>
                <ul>
                    <li><a href="https://www.github.com/ItsSammyM/mafia-rust">Github</a></li>
                    {/* eslint-disable no-script-url */}
                    {/* eslint-disable jsx-a11y/anchor-is-valid */}
                    <li><a href="javascript:" onClick={()=>{Anchor.setContent(<StandaloneWiki/>)}}>{translate("menu.wiki.title")}</a></li>
                    <li><a href="https://mafia-game-old.vercel.app/">Old Mafia</a></li>
                    <li><a href="https://netgames.io/games/">Net Games</a></li>
                    <li><a href="https://clocktower.online/">Clocktower Online</a></li>
                    <li><a href="https://secret-hitler.com/">Secret Hitler</a></li>
                </ul>
            </nav>
        </footer>
    </div>)}
}