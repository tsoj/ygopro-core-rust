//! 絢嵐たる海霊ヴァルルーン
//! Radiant Typhoon Varuroon, the Marine Eidolon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 39341885;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_RADIANT_TYPHOON}
    listed_series: &[SET_RADIANT_TYPHOON],
    // s.listed_names={CARD_MYSTICAL_SPACE_TYPHOON}
    listed_names: &[CARD_MYSTICAL_SPACE_TYPHOON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 "Radiant Typhoon" monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_RADIANT_TYPHOON),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_RADIANT_TYPHOON as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Add 1 "Mystical Space Typhoon" from your Deck or GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, c)
    })));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 2 face-up monsters on the field, including a "Radiant Typhoon" monster you control face-up in their owners' Spell & Trap Zones as Continuous Spells
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.monpltg)
    effect::set_target(d, e2, Some(cost_target(monpltg)));
    // e2:SetOperation(s.monplop)
    effect::set_operation(d, e2, Some(operation(monplop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 "Radiant Typhoon" Continuous Trap from your Deck or GY face-up on your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:IsActiveType(TYPE_QUICKPLAY) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let re = a.re.nil();
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && effect::is_active_type(d, re, TYPE_QUICKPLAY)
    })));
    // e3:SetTarget(s.trappltg)
    effect::set_target(d, e3, Some(cost_target(trappltg)));
    // e3:SetOperation(s.trapplop)
    effect::set_operation(d, e3, Some(operation(trapplop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_MYSTICAL_SPACE_TYPHOON) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_MYSTICAL_SPACE_TYPHOON]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nv = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.monplfilter(c,e)`
fn monplfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and not c:IsForbidden() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && !card::is_forbidden(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.yourradiantfilter(c,tp)`
fn yourradiantfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RADIANT_TYPHOON) and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_RADIANT_TYPHOON as u16]) && card::is_controler(d, c, tp)
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local your_g,opp_g=sg:Split(Card.IsOwner,nil,tp)
    let (your_g, opp_g) = group::split(d, sg, &|d, x| card::is_owner(d, x, tp), NO_EX);
    // return #sg==2 and sg:IsExists(s.yourradiantfilter,1,nil,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>=#your_g
    //     and Duel.GetLocationCount(1-tp,LOCATION_SZONE)>=#opp_g
    let res = sg.len() == 2
        && group::is_exists(d, sg, &|d, x| yourradiantfilter(d, x, tp), 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= your_g.len() as i32
        && duel::get_location_count(d, 1 - tp, LOCATION_SZONE as u8, None, None, None).0 >= opp_g.len() as i32;
    (res, false)
}
/// `s.monpltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn monpltg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.monplfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| monplfilter(d, x, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOFIELD)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    true
}
/// `s.monplop(e,tp,eg,ep,ev,re,r,rp)`
fn monplop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if not tc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, tc, e) {
            // local owner=tc:GetOwner()
            let owner = card::get_owner(d, tc);
            // if Duel.GetLocationCount(owner,LOCATION_SZONE)<=0 then
            if duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
                // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
                duel::send_to_grave(d, tc, REASON_RULE, None, Some(PLAYER_NONE as u8));
            // elseif Duel.MoveToField(tc,tp,owner,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
            } else {
                let is_monster = card::is_monster_card(d, tc);
                if duel::move_to_field(d, tc, tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, None) {
                    // --Treated as a Continuous Spell
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                    // e1:SetCode(EFFECT_CHANGE_TYPE)
                    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
                    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
                    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
                    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                }
            }
        }
    }
}
/// `s.trapplfilter(c,tp)`
fn trapplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RADIANT_TYPHOON) and c:IsContinuousTrap() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_RADIANT_TYPHOON as u16])
        && card::is_continuous_trap(d, c)
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.trappltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn trappltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.trapplfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| trapplfilter(d, x, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
/// `s.trapplop(e,tp,eg,ep,ev,re,r,rp)`
fn trapplop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.trapplfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let nv = aux::necro_valley_filter(|d, x| trapplfilter(d, x, tp));
    let sg = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
