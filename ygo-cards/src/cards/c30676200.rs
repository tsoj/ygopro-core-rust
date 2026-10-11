//! Hero of the Ashened City
use super::prelude::*;

pub const ID: u32 = 30676200;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBSIDIM_ASHENED_CITY, CARD_VEIDOS_ERUPTION_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
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
    // Destroy 1 Pyro monster on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(cond_main)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY),0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&faceup_obsidim), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY)`
fn faceup_obsidim(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY])
}
/// `function() return Duel.IsMainPhase() end`
fn cond_main(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `aux.FaceupFilter(Card.IsRace,RACE_PYRO)`
fn faceup_pyro(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PYRO)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsRace(RACE_PYRO) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_race(d, chkc, RACE_PYRO);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_PYRO),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_pyro), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_PYRO),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&faceup_pyro), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_OBSIDIM_ASHENED_CITY) and c:IsFieldSpell() and not c:IsForbidden()
    card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY]) && card::is_field_spell(d, c) && !card::is_forbidden(d, c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local isveidos=tc:IsCode(CARD_VEIDOS_ERUPTION_DRAGON) and tc:IsFaceup()
    let isveidos = card::is_code(d, tc, &[CARD_VEIDOS_ERUPTION_DRAGON]) && card::is_faceup(d, tc);
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and isveidos
    //   and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && isveidos
        && duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        // if fc then
        if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
            // Duel.SendtoGrave(fc,REASON_RULE)
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
        }
        // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
