import React, { ReactElement, useContext, useEffect, useState } from "react";
import "./settings.css";
import translate, { Language, languageName, LANGUAGES, switchLanguage } from "../game/lang";
import StyledText, { computeKeywordData } from "../components/StyledText";
import Icon from "../components/Icon";
import { loadSettingsParsed, RoleSpecificMenuType, saveSettings } from "../game/localStorage";
import { MobileContext, AnchorControllerContext } from "./Anchor";
import { Role, roleJsonData } from "../game/roleState.d";
import AudioController from "./AudioController";
import { getAllRoles } from "../game/roleListState.d";

export function roleSpecificMenuType(role: Role): RoleSpecificMenuType | null {
    return roleJsonData()[role].roleSpecificMenu === false ? null :
        loadSettingsParsed().roleSpecificMenus.includes(role) ? "standalone" : "playerList";
}

export default function SettingsMenu(): ReactElement {
    const [volume, setVolume] = useState<number>(loadSettingsParsed().volume);
    const [defaultName, setDefaultName] = useState<string | null>(loadSettingsParsed().defaultName);
    const [roleSpecificMenuSettings, setRoleSpecificMenuSettings] = useState(loadSettingsParsed().roleSpecificMenus);
    const mobile = useContext(MobileContext)!;
    const anchorController = useContext(AnchorControllerContext)!;

    useEffect(() => {
        AudioController.setVolume(volume);
    }, [volume]);
    
    return <div className="settings-menu-card">
        <header>
            <h1>{translate("menu.settings.title")}</h1>
        </header>
        
        <main className="settings-menu">
            <div>
                <section className="horizontal">
                    <section className="standout">
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
                    <section className="standout">
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
            <div>
                <section className="standout">
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
                    {mobile && <h2>{translate("menu.settings.advanced")}</h2>}
                    <details className="standout role-specific-menu-settings">
                        <summary>
                            {translate("menu.settings.roleSpecificMenus")}
                        </summary>
                        {
                            Object.entries(roleJsonData()).map(([role, roleJsonData]) => {
                                // const roleSpecificMenuExists = type.roleSpecificMenu;
                                const menuType: RoleSpecificMenuType = roleSpecificMenuSettings.includes(role as Role) ? "standalone" : "playerList";


                                return <div className="role-specific-menu-settings-selector" key={role} >
                                    <StyledText>{translate(`role.${role}.name`)}</StyledText>
                                    <select defaultValue={menuType} onChange={e => {
                                        let newRoleSpecificMenuSettings = [...roleSpecificMenuSettings].filter(x => 
                                            getAllRoles().includes(x)
                                        );

                                        if(e.target.options[e.target.selectedIndex].value === "playerList") {
                                            newRoleSpecificMenuSettings = [...newRoleSpecificMenuSettings].filter(x => x !== role);
                                        } else {
                                            newRoleSpecificMenuSettings = [...newRoleSpecificMenuSettings, role as Role];
                                        }
                                        setRoleSpecificMenuSettings(newRoleSpecificMenuSettings);
                                        saveSettings({ roleSpecificMenus: newRoleSpecificMenuSettings });
                                    }}>
                                        <option value="playerList">{translate("menu.settings.roleSpecificMenus.playerList")}</option>
                                        <option value="standalone">{translate("menu.settings.roleSpecificMenus.standalone")}</option>
                                    </select>
                                </div>
                            })
                        }
                    </details>
                </section>
            </div>
        </main>
    </div>
}
