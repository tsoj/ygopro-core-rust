//! バルキリー・ナイト
//! Valkyrian Knight
use super::prelude::*;

pub const ID: u32 = 99348756;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atktg)
    effect::set_value_fn(d, e1, lua_fn(atktg));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, a: Args) -> bool {
    // return c:GetCode()~=id and c:IsFaceup() and c:IsRace(RACE_WARRIOR)
    let c = a.c(1);
    card::get_code(d, c).first().copied() != Some(ID) && card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.rmfilter(c,e,tp,g)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsAbleToRemoveAsCost() and g:IsExists(s.spfilter,1,c,e,tp)
    card::is_able_to_remove_as_cost(d, c, None) && group::is_exists(d, g, &|d, x| spfilter(d, x, e, tp), 1, c.into())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsLevelAbove(5) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_effect_target(d, c, Some(e))
        && card::is_level_above(d, c, 5)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // local g=Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_GRAVE,0,e:GetHandler(),RACE_WARRIOR)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_WARRIOR)), tp, LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and g:IsExists(s.rmfilter,1,nil,e,tp,g) end
    if a.chk == Some(0) {
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(0),
        ];
        let cost_ok = aux::bfgcost().call(d, &args).truthy();
        return cost_ok
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &g, &|d, c| rmfilter(d, c, a.e, tp, &g), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=g:FilterSelect(tp,s.rmfilter,1,1,nil,e,tp,g)
    let rg = group::filter_select(d, &g, tp, &|d, c| rmfilter(d, c, a.e, tp, &g), 1, 1, false, NO_EX).nil();
    // g:Sub(rg)
    group::remove_card(&mut g, &rg);
    // rg:AddCard(e:GetHandler())
    let mut rg = rg;
    group::add_card(&mut rg, &CardSet::from_card(h));
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let sg = group::filter_select(d, &g, tp, &|d, c| spfilter(d, c, a.e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
