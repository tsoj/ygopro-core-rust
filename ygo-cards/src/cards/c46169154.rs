//! War Rock Orpis
//! ウォークライ・オーピス
use super::prelude::*;

pub const ID: u32 = 46169154;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon without Tributing
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(|d, a| sumcon(d, a))));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Send to GY and increase ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(tgcon)));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
// Normal Summon without Tributing
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsRace(RACE_WARRIOR)
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_WARRIOR)
}
/// `s.sumcon(e,c,minc,zone)`
fn sumcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 or not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil))
    a.int(2) == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(a.int(3) as u32)).0 > 0
        && (duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 || !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
// Send to GY and increase ATK
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    // return bc and bc:IsAttribute(ATTRIBUTE_EARTH) and bc:IsRace(RACE_WARRIOR)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    match bc {
        Some(bc) => card::is_attribute(d, bc, ATTRIBUTE_EARTH) && card::is_race(d, bc, RACE_WARRIOR),
        None => false,
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WAR_ROCK) and c:IsFaceup() and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    card::is_set_card(d, c, &[SET_WAR_ROCK as u16]) && card::is_faceup(d, c) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let sg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 and Duel.SendtoGrave(sg,REASON_EFFECT)>0 and sg:GetFirst():IsLocation(LOCATION_GRAVE) then
    if !sg.is_empty() && duel::send_to_grave(d, sg.clone(), REASON_EFFECT, None, None) > 0 {
        let first = group::get_first(d, &sg).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // local atkg=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,0,nil)
            let atkg = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, NO_EX);
            // if #atkg==0 then return end
            if atkg.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // for tc in aux.Next(atkg) do
            for tc in group::iter(d, &atkg) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
                effect::set_value(d, e1, 200);
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
