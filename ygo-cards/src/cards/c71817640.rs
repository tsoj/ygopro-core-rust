//! Dragonic Pendulum
use super::prelude::*;

pub const ID: u32 = 71817640;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DRACOSLAYER, SET_DRACOVERLORD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase the ATK/DEF of "Dracoslayer" monsters by 300
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DRACOSLAYER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DRACOSLAYER as u16]))));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy 1 card on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Add to hand or Special Summon 1 "Dracoslayer" or "Dracovelord" monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e5, 1, ID, 2, 0);
    // e5:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e5, Some(lua_fn(e5_cond)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end`
fn e5_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil());
    // local cont,loc,race=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_RACE)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_RACE]);
    let cont = info.first().map_or(0, |v| v.int());
    let loc = info.get(1).map_or(0, |v| v.int());
    let race = info.get(2).map_or(0, |v| v.int());
    // return re:IsMonsterEffect() and rc:IsSetCard(SET_DRACOSLAYER) and cont==tp and loc==LOCATION_MZONE and race==RACE_DRAGON
    effect::is_monster_effect(d, a.re.nil())
        && card::is_set_card(d, rc.nil(), &[SET_DRACOSLAYER as u16])
        && cont == a.tp as i64
        && loc == LOCATION_MZONE as i64
        && race == RACE_DRAGON as i64
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_DRACOSLAYER) or c:IsSetCard(SET_DRACOVERLORD)) and c:IsMonster()
    //     and (c:IsAbleToHand() or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    (card::is_set_card(d, c, &[SET_DRACOSLAYER as u16]) || card::is_set_card(d, c, &[SET_DRACOVERLORD as u16]))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // aux.ToHandOrElse(tc,tp,
    //     function() return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    let tp = a.tp;
    let e = a.e;
    aux::to_hand_or_else(
        d,
        tc,
        tp,
        Some(&|d, _c| {
            duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
        }),
        Some(&|d, _g| duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
