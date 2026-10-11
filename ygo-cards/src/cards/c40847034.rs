//! Kewl Tune Playlist
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 40847034;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KEWL_TUNE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 "Kewl Tune" monster in your field or GY; apply these effects in sequence, also for the rest of this turn after this card resolves, you cannot Special Summon, except Tuners
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(2,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 2, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// Lua call of an EVENT_BE_MATERIAL effect's target:
// tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0)
fn call_be_material_target(d: &mut Duel, eff: EffectId, tp: u8, e: EffectId) -> bool {
    match effect::get_target(d, eff) {
        None => true,
        Some(tg) => tg
            .call(
                d,
                &[
                    Value::Effect(eff),
                    Value::Int(tp as i64),
                    Value::Group(CardSet::new()),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                    Value::Effect(e),
                    Value::Int(REASON_EFFECT as i64),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                ],
            )
            .truthy(),
    }
}
// function s.applyfilter(c,e,tp)
fn applyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_KEWL_TUNE) and c:IsMonster() and c:IsFaceup()) then return false end
    if !(card::is_set_card(d, c, &[SET_KEWL_TUNE as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    // if c:IsAbleToHand() then return true end
    if card::is_able_to_hand(d, c, None) {
        return true;
    }
    // local effs={c:GetOwnEffects()}
    // for _,eff in ipairs(effs) do
    for eff in card::get_own_effects(d, c) {
        // if eff:GetCode()==EVENT_BE_MATERIAL then
        if effect::get_code(d, eff) == EVENT_BE_MATERIAL {
            // local tg=eff:GetTarget()
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            if call_be_material_target(d, eff, tp, e) {
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.applyfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| applyfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.applyfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let tcg = duel::select_target(d, tp, Some(&|d, c| applyfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tcg).nil();
    // local available_effs={}
    let mut available_effs: Vec<EffectId> = Vec::new();
    // local effs={tc:GetOwnEffects()}
    // for _,eff in ipairs(effs) do
    for eff in card::get_own_effects(d, tc) {
        // if eff:GetCode()==EVENT_BE_MATERIAL then
        if effect::get_code(d, eff) == EVENT_BE_MATERIAL {
            // local tg=eff:GetTarget()
            // if tg==nil or tg(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0) then
            if call_be_material_target(d, eff, tp, a.e) {
                // table.insert(available_effs,eff)
                available_effs.push(eff);
            }
        }
    }
    // local eff=nil
    // if #available_effs>1 then
    let eff: Option<EffectId> = if available_effs.len() > 1 {
        // local available_effs_desc={}
        // for _,eff in ipairs(available_effs) do table.insert(available_effs_desc,eff:GetDescription()) end
        let descs: Vec<u64> = available_effs.iter().map(|&x| effect::get_description(d, x)).collect();
        // local op=Duel.SelectOption(tp,table.unpack(available_effs_desc))
        let op = duel::select_option(d, tp, true, &descs);
        // eff=available_effs[op+1]
        Some(available_effs[op as usize])
    } else {
        // eff=available_effs[1]
        available_effs.first().copied()
    };
    // if eff then
    if let Some(eff) = eff {
        // Duel.Hint(HINT_OPSELECTED,1-tp,eff:GetDescription())
        duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, eff));
        // Duel.ClearTargetCard()
        duel::clear_target_card(d);
        // tc:CreateEffectRelation(e)
        card::create_effect_relation(d, tc, a.e);
        // e:SetLabel(eff:GetLabel())
        let lbl = effect::get_label(d, eff);
        effect::set_label(d, a.e, &[lbl]);
        // e:SetLabelObject(eff:GetLabelObject())
        let lo = effect::get_label_object(d, eff);
        effect::set_label_object(d, a.e, lo);
        // local tg=eff:GetTarget()
        // if tg then
        if let Some(tg) = effect::get_target(d, eff) {
            // tg(e,tp,eg,ep,ev,re,r,rp,1)
            tg.call(d, &target_args(a, Some(1), None));
            // Duel.ClearOperationInfo(0)
            duel::clear_operation_info(d, 0);
        }
    }
    // local cd=e:GetChainData()
    // cd.playlist_target_card=tc
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 100, Value::Card(tc));
    // cd.playlist_target_effect=eff
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 101, eff.map_or(Value::Nil, Value::Effect));
    // e:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, a.e, CATEGORY_TOHAND);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, tp, 0);
    true
}
// The arguments (e,tp,eg,ep,ev,re,r,rp[,chk[,chkc]]) of a Lua target call
fn target_args(a: &EffArgs, chk: Option<i64>, chkc: Option<CardId>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    if let Some(chkc) = chkc {
        v.push(Value::Card(chkc));
    }
    v
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    // local tc=cd.playlist_target_card
    let tc = chain::effect_data(d, a.e, None, chain::field::CARD + 100).as_card();
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local break_chk=false
            let mut break_chk = false;
            // local te=cd.playlist_target_effect
            // if te then
            if let Some(te) = chain::effect_data(d, a.e, None, chain::field::CARD + 101).as_effect() {
                // --● Apply that target's effect that activates when it is sent to the GY as Synchro Material
                // local op=te:GetOperation()
                // if tc:IsFaceup() and op then
                if let Some(op) = effect::get_operation(d, te) {
                    if card::is_faceup(d, tc) {
                        // op(e,tp,eg,ep,ev,re,r,rp)
                        op.call(d, &target_args(a, None, None));
                        // break_chk=true
                        break_chk = true;
                    }
                }
            }
            // --● Return that target to the hand
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // if Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 then
            if duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0 {
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, tp);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // For the rest of this turn after this card resolves, you cannot Special Summon, except Tuners
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return not c:IsType(TYPE_TUNER)
        !card::is_type(d, c, TYPE_TUNER)
    // elseif c:IsMonsterCard() then
    } else if card::is_monster_card(d, c) {
        // return not c:IsOriginalType(TYPE_TUNER)
        !card::is_original_type(d, c, TYPE_TUNER)
    } else {
        // (falls off the end: nil, read as false)
        false
    }
}
