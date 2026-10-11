//! Signs of the Tistina
use super::prelude::*;

pub const ID: u32 = 85277313;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destroy this card when target leaves the field
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(destroy_self)));
    card::register_effect(d, c, e2, false);
    // Destroy target when this card leaves the field
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e3, Some(condition(cond_turn)));
    effect::set_operation(d, e3, Some(operation(tcdesop)));
    card::register_effect(d, c, e3, false);
}
// function(e,tp) return Duel.IsTurnPlayer(1-tp) end
fn cond_turn(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
// the monster data of "Tistina" as the Lua call IsPlayerCanSpecialSummonMonster(tp,id,SET_TISTINA,TYPE_MONSTER|TYPE_EFFECT,0,0,10,RACE_AQUA,ATTRIBUTE_LIGHT)
fn tistina_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_TISTINA as u16]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(0),
        defense: Some(0),
        level: Some(10),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) and Duel.IsPlayerCanSpecialSummonMonster(...)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, tistina_data(), None, None, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !duel::is_player_can_special_summon_monster(d, tp, ID, tistina_data(), None, None, 0) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // return tc and eg:IsContains(tc) and Duel.IsTurnPlayer(1-tp)
    match tc {
        Some(tc) => group::is_contains(&a.eg.clone().nil(), tc) && duel::is_turn_player(d, 1 - a.tp),
        None => false,
    }
}
// function(e) Duel.Destroy(e:GetHandler(),REASON_EFFECT) end
fn destroy_self(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
// function s.tcdesop(e,tp,eg,ep,ev,re,r,rp)
fn tcdesop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
