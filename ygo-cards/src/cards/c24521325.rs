//! Template Skipper
use super::prelude::*;

pub const ID: u32 = 24521325;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand) to your zone a Cyberse Link Monster points to
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(POS_FACEUP,0)
    effect::set_target_range(d, e1, POS_FACEUP, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(function(e,c) return 0,aux.GetMMZonesPointedTo(c:GetControler(),aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE)) end)
    effect::set_value_fn(d, e1, lua_fn(spval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 Cyberse monster from your hand or GY, and if you do, this card can be treated as Link Material with the same name this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.rmvtg)
    effect::set_cost(d, e2, Some(cost_target(rmvtg)));
    // e2:SetOperation(s.rmvop)
    effect::set_operation(d, e2, Some(operation(rmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cybersefilter(c)
fn cybersefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsAbleToRemove() and not c:IsCode(id)
    card::is_race(d, c, RACE_CYBERSE) && card::is_able_to_remove(d, c, None, None, None) && !card::is_code(d, c, &[ID])
}
// e2:SetValue(function(e,c) return 0,aux.GetMMZonesPointedTo(...) end)
fn spval(d: &mut Duel, a: Args) -> Value {
    // c:GetControler()
    let c = a.c(1);
    let p = card::get_controler(d, c);
    // aux.GetMMZonesPointedTo(p, aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE))
    let zones = aux::get_mm_zones_pointed_to(d, p, Some(&|d, x| card::is_race(d, x, RACE_CYBERSE)), None, None, None);
    // return 0, zones
    Value::Multi(vec![Value::Int(0), Value::Int(zones as i64)])
}
// function s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cybersefilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cybersefilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.rmvop(e,tp,eg,ep,ev,re,r,rp)
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.cybersefilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| cybersefilter(d, x))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if let Some(tc) = tc {
        if duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // This card can be treated as Link Material with the same name as the banished monster if used for a Link Summon this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_ADD_CODE)
            effect::set_code(d, e1, EFFECT_ADD_CODE);
            // e1:SetOperation(function(scard,sumtype,tp) return sumtype&(SUMMON_TYPE_LINK|MATERIAL_LINK)==SUMMON_TYPE_LINK|MATERIAL_LINK end)
            effect::set_operation(d, e1, Some(lua_fn(|_d, a: Args| {
                let sumtype = a.int(1) as u64;
                (sumtype & (SUMMON_TYPE_LINK as u64 | MATERIAL_LINK)) == (SUMMON_TYPE_LINK as u64 | MATERIAL_LINK)
            })));
            // e1:SetValue(tc:GetCode())
            let code = card::get_code(d, tc).first().copied().unwrap_or(0);
            effect::set_value(d, e1, code as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
