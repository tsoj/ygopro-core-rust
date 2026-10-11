//! Ancient Gear Statue
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 32762201;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_GEAR_GOLEM, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent controls more monsters than you do, you can Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Ancient Gear Golem", or 1 monster that mentions it, from your hand or Deck, ignoring its Summoning conditions
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.hdsptg)
    effect::set_target(d, e2, Some(cost_target(hdsptg)));
    // e2:SetOperation(s.hdspop)
    effect::set_operation(d, e2, Some(operation(hdspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)<Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) < duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
}
// function s.hdspfilter(c,e,tp)
fn hdspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsCode(CARD_ANCIENT_GEAR_GOLEM) or c:ListsCode(CARD_ANCIENT_GEAR_GOLEM)) and c:IsMonster() and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    (card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) || card::lists_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]))
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.hdsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn hdsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 > 0
            // and Duel.IsExistingMatchingCard(s.hdspfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
            && duel::is_existing_matching_card(d, Some(&|d, c| hdspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.hdspop(e,tp,eg,ep,ev,re,r,rp)
fn hdspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hdspfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| hdspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
