//! Symphonic Warrior DJJ
use super::prelude::*;

pub const ID: u32 = 70204022;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYMPHONIC_WARRIOR], listed_names: &[75304793, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Flip "Symphonic Warrior" monster face-up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon self
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.sspcon)
    effect::set_condition(d, e2, Some(condition(sspcon)));
    // e2:SetTarget(s.ssptg)
    effect::set_target(d, e2, Some(cost_target(ssptg)));
    // e2:SetOperation(s.sspop)
    effect::set_operation(d, e2, Some(operation(sspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon "Symphonic Warrior" monster from Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.dsptg)
    effect::set_target(d, e3, Some(cost_target(dsptg)));
    // e3:SetOperation(s.dspop)
    effect::set_operation(d, e3, Some(operation(dspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SYMPHONIC_WARRIOR) and c:IsFacedown() and c:IsDefensePos() and c:IsCanChangePosition()
    card::is_set_card(d, c, &[SET_SYMPHONIC_WARRIOR as u16]) && card::is_facedown(d, c) && card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&posfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, a.tp, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.posfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.sspcon(e,tp,eg,ep,ev,re,r,rp)`
fn sspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_SYMPHONIC_WARRIOR)
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_SYMPHONIC_WARRIOR as u16])
}
/// `s.ssptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ssptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.sspop(e,tp,eg,ep,ev,re,r,rp)`
fn sspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.dspfilter(c,e,tp,pos)`
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, pos: u32) -> bool {
    // return c:IsSetCard(SET_SYMPHONIC_WARRIOR) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,pos)
    card::is_set_card(d, c, &[SET_SYMPHONIC_WARRIOR as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(pos as u8), None, None)
}
/// `s.dsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local pos=POS_FACEDOWN_DEFENSE
    let mut pos = POS_FACEDOWN_DEFENSE;
    // if Duel.IsEnvironment(75304793,tp,LOCATION_FZONE) then pos=pos|POS_FACEUP end
    if duel::is_environment(d, 75304793, Some(tp), Some(LOCATION_FZONE)) {
        pos |= POS_FACEUP;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.dspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,pos) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| dspfilter(d, c, a.e, tp, pos)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.dspop(e,tp,eg,ep,ev,re,r,rp)`
fn dspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local pos=POS_FACEDOWN_DEFENSE
    let mut pos = POS_FACEDOWN_DEFENSE;
    // if Duel.IsEnvironment(75304793,tp,LOCATION_FZONE) then pos=pos|POS_FACEUP end
    if duel::is_environment(d, 75304793, Some(tp), Some(LOCATION_FZONE)) {
        pos |= POS_FACEUP;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.dspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,pos):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| dspfilter(d, c, a.e, tp, pos)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,pos) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, pos as u8, None) {
            // if sc:IsFacedown() then
            if card::is_facedown(d, sc) {
                // Duel.ConfirmCards(1-tp,sc)
                duel::confirm_cards(d, 1 - tp, sc);
            // elseif sc:IsFaceup() then
            } else if card::is_faceup(d, sc) {
                // Negate its effects
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetValue(RESET_TURN_SET)
                effect::set_value(d, e2, RESET_TURN_SET as i64);
                // sc:RegisterEffect(e2)
                card::register_effect(d, sc, e2, false);
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
