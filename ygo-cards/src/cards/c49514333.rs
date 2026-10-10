//! Tiki Soul
use super::prelude::*;

pub const ID: u32 = 49514333;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set Trap cards that are monsters instead of sending them to the GY if they are destroyed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_SEND_REPLACE)
    effect::set_code(d, e2, EFFECT_SEND_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,1800,4,RACE_ROCK,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, card_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,1800,4,RACE_ROCK,ATTRIBUTE_LIGHT)`
fn card_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(1000),
        defense: Some(1800),
        level: Some(4),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !duel::is_player_can_special_summon_monster(d, tp, ID, card_data(), None, None, 0) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ... (the replaced-card group and this card)
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local count=eg:FilterCount(s.repfilter,e:GetHandler(),tp)
        let count = group::filter_count(d, &eg, &|d, c| repfilter(d, c, a.tp), h.into());
        // return count>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>=count
        return count > 0 && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 >= count as i32;
    }
    // if Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // local replcg=e:GetLabelObject()
        let mut replcg = effect::get_label_object(d, a.e).as_group().cloned().nil();
        // replcg:Clear()
        replcg.clear();
        // local g=eg:Filter(s.repfilter,e:GetHandler(),tp)
        let g = group::filter(d, &eg, &|d, c| repfilter(d, c, a.tp), h.into());
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
        // Duel.ChangePosition(g,POS_FACEDOWN)
        duel::change_position(d, g.clone(), POS_FACEDOWN as u8, None, None, None, false);
        // Duel.RaiseEvent(g,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, g.clone(), EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
        // replcg:Merge(g)
        group::add_card(&mut replcg, &g);
        // The label object is the group that replcg refers to: store the change back
        effect::set_label_object(d, a.e, Value::Group(replcg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOriginalType(TYPE_TRAP) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()
    //     and c:IsReason(REASON_DESTROY) and c:GetDestination()==LOCATION_GRAVE
    //     and c:GetReasonPlayer()==1-tp and c:GetOwner()==tp and c:IsCanTurnSet()
    card::is_original_type(d, c, TYPE_TRAP)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::get_destination(d, c) == LOCATION_GRAVE
        && card::get_reason_player(d, c) == 1 - tp
        && card::get_owner(d, c) == tp
        && card::is_can_turn_set(d, c)
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return e:GetLabelObject():IsContains(c)
    let c = a.c(1);
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned().nil();
    group::is_contains(&g, c)
}
