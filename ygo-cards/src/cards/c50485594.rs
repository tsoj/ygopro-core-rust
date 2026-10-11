//! Rescue Hamster
use super::prelude::*;

pub const ID: u32 = 50485594;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN, SET_ODD_EYES], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e4, Some(cost::self_tribute()));
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c,g)`: a card in `g` with the same code as `c` (other than `c`)
fn filter2(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsExists(Card.IsCode,1,c,c:GetCode())
    let codes = card::get_code(d, c);
    group::is_exists(d, g, &|d, x| card::is_code(d, x, &codes), 1, c.into())
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // return g:IsExists(s.filter2,1,nil,g)
        return group::is_exists(d, &g, &|d, x| filter2(d, x, &g), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local sg=g:Filter(s.filter2,nil,g)
    let sg = group::filter(d, &g, &|d, x| filter2(d, x, &g), NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local hg1=sg:Select(tp,1,1,nil)
    let mut hg1 = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
    // local hg2=sg:Filter(Card.IsCode,hg1:GetFirst(),hg1:GetFirst():GetCode())
    let tc1 = group::get_first(d, &hg1).nil();
    let codes = card::get_code(d, tc1);
    let hg2 = group::filter(d, &sg, &|d, x| card::is_code(d, x, &codes), tc1.into(), );
    // hg1:AddCard(hg2:GetFirst())
    let tc2 = group::get_first(d, &hg2).nil();
    group::add_card(&mut hg1, &CardSet::from_card(tc2));
    // Duel.SendtoHand(hg1,nil,REASON_EFFECT)
    duel::send_to_hand(d, hg1.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,hg1)
    duel::confirm_cards(d, 1 - a.tp, hg1);
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsLevelBelow(5)
    //   and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,2,nil,c:GetCode(),e,tp)
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_level_below(d, c, 5)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, code, e, tp)), tp, LOCATION_DECK, 0, 2, NO_EX)
}
/// `s.spfilter2(c,code,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return ft > 1
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local code=g:GetFirst():GetCode()
    let first = group::get_first(d, &g).nil();
    let code = card::get_code(d, first)[0];
    // local dg=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_DECK,0,nil,code,e,tp)
    let dg = duel::get_matching_group(d, Some(&|d, c| spfilter2(d, c, code, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #dg>1 then
    if dg.len() > 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg1=dg:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        // local tc1=sg1:GetFirst()
        let tc1 = group::get_first(d, &sg1).nil();
        // local sg2=dg:Filter(Card.IsCode,tc1,tc1:GetCode())
        let codes1 = card::get_code(d, tc1);
        let sg2 = group::filter(d, &dg, &|d, x| card::is_code(d, x, &codes1), tc1.into(), );
        // local tc2=sg2:GetFirst()
        let tc2 = group::get_first(d, &sg2);
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,tp,false,false,POS_FACEUP)
        let tc2 = tc2.nil();
        duel::special_summon_step(d, tc2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // tc2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // sg1:KeepAlive()  (nothing in Rust)
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetLabelObject(sg1)
        effect::set_label_object(d, e1, Value::Group(sg1.clone()));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e2)
        card::register_effect(d, tc1, e2, false);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // tc2:RegisterEffect(e3)
        card::register_effect(d, tc2, e3, false);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e4, EFFECT_DISABLE_EFFECT);
        // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e4)
        card::register_effect(d, tc1, e4, false);
        // local e5=e4:Clone()
        let e5 = effect::clone(d, e4);
        // tc2:RegisterEffect(e5)
        card::register_effect(d, tc2, e5, false);
    }
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup()  (nothing in Rust)
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, fid), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
