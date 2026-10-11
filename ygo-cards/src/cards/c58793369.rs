//! Drytron Fafnir
use super::prelude::*;

pub const ID: u32 = 58793369;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DRYTRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent effect and activation from being negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetValue(s.effectfilter)
    effect::set_value_fn(d, e2, lua_fn(effectfilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e3, EFFECT_CANNOT_DISEFFECT);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetValue(s.effectfilter)
    effect::set_value_fn(d, e3, lua_fn(effectfilter));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Decrease Levels
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e4, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e4:SetCondition(s.lvcond)
    effect::set_condition(d, e4, Some(condition(lvcond)));
    // e4:SetTarget(s.lvtg)
    effect::set_target(d, e4, Some(cost_target(lvtg)));
    // e4:SetOperation(s.lvop)
    effect::set_operation(d, e4, Some(operation(lvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_DRYTRON) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_DRYTRON as u16])
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
// function s.effectfilter(e,ct)
fn effectfilter(d: &mut Duel, a: Args) -> bool {
    let ct = a.int(1) as u8;
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // return te:GetHandler():IsRitualSpell()
    let h = effect::get_handler(d, te).nil();
    card::is_ritual_spell(d, h)
}
// function s.lvcond(e,tp,eg,ep,ev,re,r,rp)
fn lvcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DRYTRON),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DRYTRON as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
// function s.lvfilter(c)
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsLevelAbove(2) and c:IsAttackAbove(1000)
    card::is_faceup(d, c) && card::has_level(d, c) && card::is_level_above(d, c, 2) && card::is_attack_above(d, c, 1000)
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg and eg:IsExists(s.lvfilter,1,nil) end
    if a.chk == Some(0) {
        return match a.eg.clone() {
            Some(eg) => group::is_exists(d, &eg, &lvfilter, 1, NO_EX),
            None => false,
        };
    }
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or not eg then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let eg = match a.eg.clone() {
        Some(eg) => eg,
        None => return,
    };
    // local g=eg:Filter(s.lvfilter,nil)
    let g = group::filter(d, &eg, &lvfilter, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // local lv=tc:GetAttack()//1000
            let lv = card::get_attack(d, tc).div_euclid(1000);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_LEVEL)
            effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
            // e1:SetValue(-lv)
            effect::set_value(d, e1, -(lv as i64));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
