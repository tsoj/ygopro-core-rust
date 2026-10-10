//! Herald of the Abyss
use super::prelude::*;

pub const ID: u32 = 94016752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Make your opponent send a monster of the declared Type and Attribute they control to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX))));
    // e1:SetCost(Cost.PayLP(1500))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1500))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RACE as u64);
    // local race=Duel.AnnounceRace(tp,1,RACE_ALL)
    let race = duel::announce_race(d, a.tp, 1, RACE_ALL);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // local attr=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let attr = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    // e:SetLabel(race,attr)
    effect::set_label(d, a.e, &[race as i64, attr as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.tgfilter(c,race,attr)`
fn tgfilter(d: &mut Duel, c: CardId, race: u64, attr: u32) -> bool {
    // return c:IsFaceup() and c:IsRace(race) and c:IsAttribute(attr) and c:IsAbleToGrave()
    card::is_faceup(d, c) && card::is_race(d, c, race) && card::is_attribute(d, c, attr) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local race,attr=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels[0] as u64;
    let attr = labels.get(1).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(1-tp,s.tgfilter,tp,0,LOCATION_MZONE,1,1,nil,race,attr):GetFirst()
    let sel = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| tgfilter(d, c, race, attr)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // Duel.HintSelection(sc,true)
    duel::hint_selection(d, CardSet::from_card(sc), true);
    // local code=sc:GetCode()
    let code = card::get_code(d, sc)[0];
    // if Duel.SendtoGrave(sc,REASON_RULE,PLAYER_NONE,1-tp)>0 then
    if duel::send_to_grave(d, CardSet::from_card(sc), REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp)) > 0 {
        // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetLabel(code)
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and (rc:HasFlagEffect(id) or rc:IsCode(e:GetLabel()))
    effect::is_monster_effect(d, re) && (card::has_flag_effect(d, rc, ID, None) || card::is_code(d, rc, &[effect::get_label(d, e) as u32]))
}
