//! Supreme King Dragon Lightwurm
use super::prelude::*;

pub const ID: u32 = 41908872;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ZARC],
    listed_series: &[SET_SUPREME_KING_GATE, SET_SUPREME_KING_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Supreme King Dragon" or "Supreme King Gate" Pendulum Monster to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add this card to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e4, LOCATION_EXTRA);
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(s.ethcon)
    effect::set_condition(d, e4, Some(condition(ethcon)));
    // e4:SetTarget(s.ethtg)
    effect::set_target(d, e4, Some(cost_target(ethtg)));
    // e4:SetOperation(s.ethop)
    effect::set_operation(d, e4, Some(operation(ethop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.zarcfilter(c,tp)
fn zarcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_ZARC)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_PENDULUM),tp,LOCATION_MZONE,0,1,c)
    card::is_faceup(d, c)
        && card::is_code(d, c, &[CARD_ZARC])
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_PENDULUM)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.zarcfilter,tp,LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| zarcfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// s.pendfilter(c,att,lv)
fn pendfilter(d: &mut Duel, c: CardId, att: u32, lv: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and not (c:GetAttribute()==att and c:IsLevel(lv))
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && !(card::get_attribute(d, c) == att && card::is_level(d, c, &[lv]))
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.pendfilter,tp,LOCATION_MZONE,0,c,c:GetAttribute(),c:GetLevel())
    let catt = card::get_attribute(d, c);
    let clv = card::get_level(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| pendfilter(d, x, catt, clv)), tp, LOCATION_MZONE, 0, c.into());
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local sg=g:Select(tp,1,1,nil)
    let mut sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg,true)
    duel::hint_selection(d, sg.clone(), true);
    // sg:AddCard(c)
    group::add_card(&mut sg, &CardSet::from_card(c));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
    // local sc=sg:Select(tp,1,1,nil):GetFirst()
    let scg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &scg);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc,true)
        duel::hint_selection(d, CardSet::from_card(sc), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local tc=(sg-sc):GetFirst()
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &CardSet::from_card(sc));
        let tc = group::get_first(d, &rest);
        // --Copy attribute and level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(sc:GetAttribute())
        let sattr = card::get_attribute(d, sc);
        effect::set_value(d, e1, sattr as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
        // e2:SetValue(sc:GetLevel())
        let slv = card::get_level(d, sc);
        effect::set_value(d, e2, slv as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc.nil(), e2, false);
    }
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_SUPREME_KING_GATE,SET_SUPREME_KING_DRAGON}) and c:IsAbleToHand()
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SUPREME_KING_GATE as u16, SET_SUPREME_KING_DRAGON as u16])
        && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// s.exfilter(c)
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUPREME_KING_DRAGON) and (c:IsSynchroSummonable() or c:IsXyzSummonable())
    card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])
        && (card::is_synchro_summonable(d, c, None, None, 0, 0) || card::is_xyz_summonable(d, c, None, None, 0, 0))
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 or not g:GetFirst():IsLocation(LOCATION_HAND) then return end
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // local sg=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_EXTRA,0,nil)
    let sg = duel::get_matching_group(d, Some(&exfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,5)) then return end
    if sg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=sg:Select(tp,1,1,nil):GetFirst()
    let scg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &scg) {
        Some(sc) => sc,
        None => return,
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if sc:IsType(TYPE_SYNCHRO) then
    if card::is_type(d, sc, TYPE_SYNCHRO) {
        // Duel.SynchroSummon(tp,sc)
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    // elseif sc:IsType(TYPE_XYZ) then
    } else if card::is_type(d, sc, TYPE_XYZ) {
        // Duel.XyzSummon(tp,sc)
        duel::xyz_summon(d, tp, sc, None, None, 0, 0);
    }
}
// s.ethfilter(c,tp)
fn ethfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:GetPreviousTypeOnField()&TYPE_PENDULUM==TYPE_PENDULUM
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && (card::get_previous_type_on_field(d, c) & TYPE_PENDULUM) == TYPE_PENDULUM
}
// s.ethcon(e,tp,eg,ep,ev,re,r,rp)
fn ethcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.ethfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| ethfilter(d, c, a.tp), 1, NO_EX)
}
// s.ethtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ethtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// s.ethop(e,tp,eg,ep,ev,re,r,rp)
fn ethop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
