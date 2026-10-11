//! Yosenju Oroshi Channeling
use super::prelude::*;

pub const ID: u32 = 61884774;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YOSENJU],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_LIMIT_ZONE)
    effect::set_property(d, e1, EFFECT_FLAG_LIMIT_ZONE, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetValue(s.zones)
    effect::set_value_fn(d, e1, lua_fn(zones));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.zones(e,tp,eg,ep,ev,re,r,rp)`
fn zones(d: &mut Duel, a: Args) -> i64 {
    // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) and 0xff or 0xe
    let tp = a.p(1);
    if duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX) {
        0xff
    } else {
        0xe
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YOSENJU) and c:IsLevelAbove(5) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_YOSENJU as u16]) && card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
/// `s.pzfilter(c,cd)`
fn pzfilter(d: &mut Duel, c: CardId, cd: u32) -> bool {
    // return c:IsCode(cd) and not c:IsForbidden()
    card::is_code(d, c, &[cd]) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=(Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,1,nil,65025250)
    //     and Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,1,nil,91420254)
    //     and Duel.CheckLocation(tp,LOCATION_PZONE,0) and Duel.CheckLocation(tp,LOCATION_PZONE,1))
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| pzfilter(d, c, 65025250)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| pzfilter(d, c, 91420254)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::check_location(d, tp, LOCATION_PZONE, 0)
        && duel::check_location(d, tp, LOCATION_PZONE, 1);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]);
    // else
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) + 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==0 then
    if op == 0 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_YOSENJU)
    !card::is_set_card(d, c, &[SET_YOSENJU as u16])
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if Duel.IsTurnPlayer(tp) then return false end
    if duel::is_turn_player(d, a.tp) {
        return false;
    }
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    // else return true end
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.splimit)
        effect::set_target(d, e1, Some(effect_card_fn(splimit)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==0 then
    if op == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else {
        // if not Duel.CheckPendulumZones(tp) then return end
        if !duel::check_pendulum_zones(d, tp) {
            return;
        }
        // local tc1=Duel.GetFirstMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,nil,65025250)
        let tc1 = duel::get_first_matching_card(d, Some(&|d, c| pzfilter(d, c, 65025250)), tp, LOCATION_DECK, 0, NO_EX);
        // local tc2=Duel.GetFirstMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,nil,91420254)
        let tc2 = duel::get_first_matching_card(d, Some(&|d, c| pzfilter(d, c, 91420254)), tp, LOCATION_DECK, 0, NO_EX);
        // if not (tc1 and tc2) then return end
        let (tc1, tc2) = match (tc1, tc2) {
            (Some(a1), Some(a2)) => (a1, a2),
            _ => return,
        };
        // Duel.MoveToField(tc1,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc1, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        // Duel.MoveToField(tc2,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc2, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // tc1:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,0,1,fid)
        card::register_flag_effect(d, tc1, ID, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 0, 1, fid, 0);
        // tc2:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,0,1,fid)
        card::register_flag_effect(d, tc2, ID, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 0, 1, fid, 0);
        // local sg=Group.FromCards(tc1,tc2)
        let mut sg = CardSet::from_card(tc1);
        group::add_card(&mut sg, &CardSet::from_card(tc2));
        // sg:KeepAlive()
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid]);
        // e1:SetLabelObject(sg)
        effect::set_label_object(d, e1, Value::Group(sg));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
