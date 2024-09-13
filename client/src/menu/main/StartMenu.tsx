import React, { ReactElement, useContext } from "react";
import GAME_MANAGER from "../../index";
import "../../index.css"
import "./startMenu.css"
import translate from "../../game/lang";
import { AnchorControllerContext, MobileContext } from "../Anchor";
import PlayMenu from "./PlayMenu";
import LoadingScreen from "../LoadingScreen";
import GameModesEditor from "../../components/gameModeSettings/GameModesEditor";
import Icon from "../../components/Icon";
import SettingsMenu from "../Settings";
import StandaloneWiki from "./StandaloneWiki";

export default function StartMenu(): ReactElement {
    const mobile = useContext(MobileContext)!;
    const { setContent: setAnchorContent, setCoverCard } = useContext(AnchorControllerContext)!;
    return <div className="sm">
        <main>
            <section id="main">
                {
                    mobile ? 
                    <h2>{translate("menu.start.title")}</h2> :
                    <h1>{translate("menu.start.title")}</h1>
                }
                <div>
                    <button onClick={async () => {
                        setAnchorContent(<LoadingScreen type="default"/>);
                        await GAME_MANAGER.setOutsideLobbyState();
                        setAnchorContent(<PlayMenu/>);
                    }}>
                        <Icon>play_arrow</Icon> {translate("menu.start.button.play")}
                    </button>
                    <button onClick={() => setCoverCard(<SettingsMenu />)}>
                        <Icon>settings</Icon> {translate("menu.settings.title")}
                    </button>
                    <button onClick={() => setCoverCard(<GameModesEditor/>)}>
                        <Icon>edit</Icon> {translate("menu.globalMenu.gameSettingsEditor")}
                    </button>
                    <button onClick={() => setAnchorContent(<StandaloneWiki/>)}>
                        <Icon>menu_book</Icon> {translate("menu.wiki.title")}
                    </button>
                </div>
                
            </section>
        </main>
        <footer>
            <nav>
                <ul>
                    <li><a href="https://www.github.com/ItsSammyM/mafia-rust">Github</a></li>
                    <li><a href="https://discord.gg/Vxw7gFPfJj">Discord</a></li>
                    {/* eslint-disable no-script-url */}
                    {/* eslint-disable jsx-a11y/anchor-is-valid */}
                    <li><a href="https://netgames.io/games/">Net Games</a></li>
                    <li><a href="https://clocktower.online/">Clocktower Online</a></li>
                    <li><a href="https://secret-hitler.com/">Secret Hitler</a></li>
                </ul>
            </nav>
        </footer>
    </div>
}
