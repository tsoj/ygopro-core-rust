//! Doll House
use super::prelude::*;

pub const ID: u32 = 67331360;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_PRINCESS_COLOGNE, CARD_GRANDPA_DEMETTO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 monster from your Deck with the same name as each target, as a Level 6 DARK monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 "Grandpa Demetto" you control to "Princess Cologne" you control as material, then end the Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let att = duel::get_attacker(d).nil();
        card::is_controler(d, att, 1 - a.tp)
    })));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsType(TYPE_NORMAL) and (c:IsAttack(0) or c:IsDefense(0)) and c:IsCanBeEffectTarget(e)
    card::is_type(d, c, TYPE_NORMAL)
        && (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0]))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.resconfunc(cg)`: returns the rescon function for Auxiliary.SelectUnselectGroup
fn resconfunc(cg: CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    // return function (sg,e,tp,mg)
    move |d, sg, _e, _tp, _mg, _c| (rescon_body(d, sg, &cg), false)
}
/// The body of the function returned by `s.resconfunc` (a Lua `return` of nil reads as false).
fn rescon_body(d: &mut Duel, sg: &CardSet, cg: &CardSet) -> bool {
    // local code0=sg:GetFirst():GetCode()
    let first = group::get_first(d, sg).nil();
    let code0 = card::get_code(d, first)[0];
    // local f1=cg:Filter(Card.IsCode,nil,code0)
    let f1 = group::filter(d, cg, &|d, c| card::is_code(d, c, &[code0]), NO_EX);
    // if #f1<1 then return end
    if f1.is_empty() {
        return false;
    }
    // if #sg>1 then
    if sg.len() > 1 {
        // local code1=sg:GetNext():GetCode()
        let second = group::iter(d, sg).get(1).copied().nil();
        let code1 = card::get_code(d, second)[0];
        // return (code0==code1 and #f1>1) or (cg-f1):IsExists(Card.IsCode,1,nil,code1)
        if code0 == code1 && f1.len() > 1 {
            return true;
        }
        let mut diff = cg.clone();
        group::remove_card(&mut diff, &f1);
        return group::is_exists(d, &diff, &|d, c| card::is_code(d, c, &[code1]), 1, NO_EX);
    }
    // return true
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:AssumeProperty(ASSUME_LEVEL,6)
    card::assume_property(d, c, ASSUME_LEVEL, 6);
    // c:AssumeProperty(ASSUME_ATTRIBUTE,ATTRIBUTE_DARK)
    card::assume_property(d, c, ASSUME_ATTRIBUTE, ATTRIBUTE_DARK as u64);
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local tg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_GRAVE,0,nil,e)
    let tg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local rescon=s.resconfunc(Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp))
    let spg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    let rescon = resconfunc(spg);
    // if chk==0 then return ft>0 and aux.SelectUnselectGroup(tg,e,tp,1,1,rescon,0) end
    if a.chk == Some(0) {
        return ft > 0
            && matches!(
                aux::select_unselect_group(d, &tg, Some(e), tp, Some(1), Some(1), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_PRINCESS_COLOGNE),tp,LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and aux.SelectUnselectGroup(tg,e,tp,1,2,rescon,0) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_PRINCESS_COLOGNE])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && matches!(
            aux::select_unselect_group(d, &tg, Some(e), tp, Some(1), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
    {
        // ft=math.min(2,ft)
        ft = ft.min(2);
    } else {
        // ft=1
        ft = 1;
    }
    // local g=aux.SelectUnselectGroup(tg,e,tp,1,ft,rescon,1,tp,HINTMSG_TARGET)
    let g = aux::select_unselect_group(d, &tg, Some(e), tp, Some(1), Some(ft as usize), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,#g,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, g.len() as u8, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // local gc=#g
    let gc = g.len();
    // if gc==0 then return end
    if gc == 0 {
        return;
    }
    // local rescon=s.resconfunc(g)
    let rescon = resconfunc(g.clone());
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<gc
    //     or (gc>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     or not aux.SelectUnselectGroup(sg,e,tp,gc,gc,rescon,0) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < gc as i32
        || (gc > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
        || !matches!(
            aux::select_unselect_group(d, &sg, Some(e), tp, Some(gc), Some(gc), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
    {
        return;
    }
    // local ssg=aux.SelectUnselectGroup(sg,e,tp,gc,gc,rescon,1,tp,HINTMSG_SPSUMMON)
    let ssg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(gc), Some(gc), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #g==#ssg then
    if g.len() == ssg.len() {
        // for sc in ssg:Iter() do
        for sc in group::iter(d, &ssg) {
            // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // Change its Attribute and Level
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
                effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
                // e1:SetValue(ATTRIBUTE_DARK)
                effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1,true)
                card::register_effect(d, sc, e1, true);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_CHANGE_LEVEL)
                effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
                // e2:SetValue(6)
                effect::set_value(d, e2, 6);
                // sc:RegisterEffect(e2,true)
                card::register_effect(d, sc, e2, true);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.attachfilter(c,tp)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_GRANDPA_DEMETTO) and c:IsFaceup() and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,c,tp)
    card::is_code(d, c, &[CARD_GRANDPA_DEMETTO])
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.xyzfilter(c,mc,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_PRINCESS_COLOGNE) and c:IsFaceup() and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_code(d, c, &[CARD_PRINCESS_COLOGNE])
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // (falls off the end: the value is not used)
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local attach_c=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| attachfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let attach_c = group::get_first(d, &sel);
    // if not attach_c then return end
    let Some(attach_c) = attach_c else {
        return;
    };
    // Duel.HintSelection(attach_c)
    duel::hint_selection(d, CardSet::from_card(attach_c), true);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local xyz_c=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,attach_c,tp):GetFirst()
    let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, attach_c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let xyz_c = group::get_first(d, &sel2);
    // if xyz_c then
    if let Some(xyz_c) = xyz_c {
        // Duel.HintSelection(xyz_c)
        duel::hint_selection(d, CardSet::from_card(xyz_c), true);
        // if not attach_c:IsImmuneToEffect(e) and not xyz_c:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, attach_c, e) && !card::is_immune_to_effect(d, xyz_c, e) {
            // Duel.Overlay(xyz_c,attach_c,true)
            duel::overlay(d, xyz_c, CardSet::from_card(attach_c), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
            duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        }
    }
}
