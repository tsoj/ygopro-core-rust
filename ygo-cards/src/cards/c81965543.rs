//! Distrust Paranoia
use super::prelude::*;

pub const ID: u32 = 81965543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control of all your opponent's monsters in this card's column
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Set card in its owner's control has left the field by an opponent's effect, and is now in the GY or banished: You can Special Summon this card as an Effect Monster (Fiend/DARK/Level 10/ATK 4000/DEF 4000) with the following effect (this card is NOT treated as a Trap)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // ● Unaffected by your opponent's card effects activated in the same column
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
    })));
    // e3:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and e:GetHandler():IsColumn(te:GetCardSequence(),te:GetCardControler(),te:GetCardLocation()) end)
    effect::set_value_fn(d, e3, lua_fn(e3_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ctrlfilter(c,tp)`
fn ctrlfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsLocation(LOCATION_MZONE)
    card::is_controler(d, c, 1 - tp) && card::is_location(d, c, LOCATION_MZONE)
}
/// `local colg=c:GetColumnGroup():Match(s.ctrlfilter,nil,tp)` and
/// `local ctrl_colg=colg:Filter(Card.IsAbleToChangeControler,nil)`
fn column_groups(d: &mut Duel, c: CardId, tp: u8) -> (CardSet, CardSet) {
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::match_(d, &mut colg, &|d, x| ctrlfilter(d, x, tp), NO_EX);
    let ctrl_colg = group::filter(d, &colg, &|d, x| card::is_able_to_change_controler(d, x), NO_EX);
    (colg, ctrl_colg)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let (colg, ctrl_colg) = column_groups(d, c, tp);
    let colg_count = colg.len();
    let ctrl_colg_count = ctrl_colg.len();
    // if chk==0 then return #colg>0 and ctrl_colg_count==#colg and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>=ctrl_colg_count end
    if a.chk == Some(0) {
        return colg_count > 0
            && ctrl_colg_count == colg_count
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 >= ctrl_colg_count as i32;
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,ctrl_colg,ctrl_colg_count,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(ctrl_colg), ctrl_colg_count as u8, tp, 0);
    true
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
    let (colg, ctrl_colg) = column_groups(d, c, tp);
    let colg_count = colg.len();
    let ctrl_colg_count = ctrl_colg.len();
    // if ctrl_colg_count>0 and ctrl_colg_count==#colg then
    if ctrl_colg_count > 0 && ctrl_colg_count == colg_count {
        // Duel.GetControl(ctrl_colg,tp)
        duel::get_control(d, ctrl_colg, tp, 0, 0, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEDOWN) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD) and rp==1-tp
    //     and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    card::is_previous_position(d, c, POS_FACEDOWN)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && a.rp == 1 - tp
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,4000,4000,10,RACE_FIEND,ATTRIBUTE_DARK)`
fn can_sp_summon_self(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_EFFECT),
            attack: Some(4000),
            defense: Some(4000),
            level: Some(10),
            race: Some(RACE_FIEND),
            attribute: Some(ATTRIBUTE_DARK),
        },
        None,
        None,
        0,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_sp_summon_self(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, c.map(CardSet::from_card), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if card::is_relate_to_effect(d, c, a.e) && can_sp_summon_self(d, tp) {
        // c:AddMonsterAttribute(TYPE_EFFECT)
        card::add_monster_attribute(d, c, TYPE_EFFECT, 0, 0, 0, 0, 0);
        // c:AssumeProperty(ASSUME_RACE,RACE_FIEND)
        card::assume_property(d, c, ASSUME_RACE, RACE_FIEND);
        // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated()
///     and e:GetHandler():IsColumn(te:GetCardSequence(),te:GetCardControler(),te:GetCardLocation()) end`
fn e3_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
        && effect::is_activated(d, te)
        && {
            let h = effect::get_handler(d, e).nil();
            let seq = chain::effect_get_card(d, te, chain::Prop::Sequence).int() as u32;
            let tp = chain::effect_get_card(d, te, chain::Prop::Controler).int() as u8;
            let loc = chain::effect_get_card(d, te, chain::Prop::Location).int() as u32;
            card::is_column(d, h, seq, Some(tp), Some(loc))
        }
}
