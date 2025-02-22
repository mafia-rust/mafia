import React, { ReactElement, useContext, useEffect, useState } from "react";
import "./settings.css";
import translate, { Language, languageName, LANGUAGES, switchLanguage } from "../game/lang";
import StyledText, { computeKeywordData } from "../components/StyledText";
import Icon from "../components/Icon";
import { loadSettingsParsed, saveSettings } from "../game/localStorage";
import { AnchorControllerContext, ANCHOR_CONTROLLER } from "./Anchor";
import AudioController from "./AudioController";
import CheckBox from "../components/CheckBox";
import { DragAndDrop } from "../components/DragAndDrop";
import { MENU_THEMES, MENU_TRANSLATION_KEYS } from "./game/GameScreen";

export default function SettingsMenu(): ReactElement {
    const [volume, setVolume] = useState<number>(loadSettingsParsed().volume);
    const [fontSizeState, setFontSize] = useState<number>(loadSettingsParsed().fontSize);
    const [defaultName, setDefaultName] = useState<string | null>(loadSettingsParsed().defaultName);
    const [accessibilityFontEnabled, setAccessibilityFontEnabled] = useState(loadSettingsParsed().accessibilityFont);
    const [menuOrder, setMenuOrder] = useState(loadSettingsParsed().menuOrder);
    const [maxMenus, setMaxMenus] = useState(loadSettingsParsed().maxMenus);
    const anchorController = useContext(AnchorControllerContext)!;

    useEffect(() => {
        AudioController.setVolume(volume);
        ANCHOR_CONTROLLER?.setFontSize(fontSizeState);
        ANCHOR_CONTROLLER?.setAccessibilityFontEnabled(accessibilityFontEnabled);
    }, [volume, fontSizeState, accessibilityFontEnabled]);

    return <div className="settings-menu-card">
        <header>
            <h1>{translate("menu.settings.title")}</h1>
        </header>
        
        <main className="settings-menu">
            <div className="graveyard-menu-colors">
                <h2>{translate("menu.settings.general")}</h2>
                <section>
                    <h2><Icon size="small">volume_up</Icon> {translate("menu.settings.volume")}</h2>
                    <input className="settings-volume" type="range" min="0" max="1" step="0.01" 
                        value={volume} 
                        onChange={(e) => {
                            const volume = parseFloat(e.target.value);
                            saveSettings({volume});
                            setVolume(volume);
                        }
                    }/>
                </section>
                <section>
                    <h2>{translate("menu.settings.font")}</h2>
                    <label>
                        {translate("menu.settings.fontSize")}
                        <input type="number" min="0.5" max="2" step="0.1"
                            value={fontSizeState}
                            onChange={(e)=>{
                                if(e.target.value === "") return;
                                const fontSize = parseFloat(e.target.value);
                                if(fontSize < 0.5 || fontSize > 2) return;
                                saveSettings({fontSize});
                                setFontSize(fontSize);
                            }}
                        />
                    </label>
                    <label>
                        {translate("menu.settings.accessibilityFont")}
                        <CheckBox checked={accessibilityFontEnabled} onChange={(checked: boolean) => {
                            setAccessibilityFontEnabled(checked);
                            saveSettings({accessibilityFont: checked});
                        }}></CheckBox>
                    </label>
                </section>
                <section>
                    <h2><Icon size="small">language</Icon> {translate("menu.settings.language")}</h2>
                    <select 
                        name="lang-select" 
                        defaultValue={loadSettingsParsed().language}
                        onChange={e => {
                            const language = e.target.options[e.target.selectedIndex].value as Language;
                            switchLanguage(language);
                            saveSettings({language});
                            computeKeywordData()
                            anchorController.reload();
                        }}
                    >
                        {LANGUAGES.map(lang => <option key={lang} value={lang}>{languageName(lang)}</option>)}
                    </select>
                </section>
            </div>
            <div className="chat-menu-colors">
                <h2>{translate("menu.settings.gameplay")}</h2>
                <section>
                    <h2>{translate("menu.settings.menus")}</h2>
                    <label>
                        {translate("menu.settings.maxMenus")}
                        <input type="number" min="1" max="6" step="1"
                            value={maxMenus}
                            onChange={(e)=>{
                                if(e.target.value === "") return;
                                const maxMenus = parseFloat(e.target.value);
                                if(Math.floor(maxMenus) !== maxMenus || maxMenus < 1 || maxMenus > 6) return;
                                saveSettings({maxMenus});
                                setMaxMenus(maxMenus);
                            }}
                        />
                    </label>
                    <div>
                        {translate("menu.settings.menuOrder")}
                        <div className="menu-list">
                            <DragAndDrop
                                items={menuOrder}
                                render={menu => <div className={"placard " + (MENU_THEMES[menu] ?? "")}>
                                    {translate(MENU_TRANSLATION_KEYS[menu] + ".icon")}
                                </div>}
                                onDragEnd={newItems => {
                                    saveSettings({menuOrder: [...newItems]})
                                    setMenuOrder([...newItems])
                                }}
                            />
                        </div>
                    </div>
                </section>
                <section>
                    <h2>{translate("menu.settings.defaultName")}</h2>
                    <input type="text"
                        value={defaultName===null?"":defaultName} 
                        placeholder={translate("menu.lobby.field.namePlaceholder")}
                        onChange={(e) => {
                            const defaultName = e.target.value === "" ? null : e.target.value;
                            saveSettings({defaultName});
                            setDefaultName(defaultName);
                        }
                    }/>
                </section>
                <section>
                    <h2><StyledText className="keyword-evil">{translate("menu.settings.dangerZone")}</StyledText></h2>
                    <button onClick={()=>{
                        if(!window.confirm(translate("confirmDelete"))) return;
                        localStorage.clear();
                        anchorController.clearCoverCard();
                    }}><Icon>delete_forever</Icon> {translate('menu.settings.eraseSaveData')}</button>
                </section>
            </div>
        </main>
    </div>
}
