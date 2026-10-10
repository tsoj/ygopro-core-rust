//! Supreme King Gate Zero
use super::prelude::*;

pub const ID: u32 = 96227613;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUSION], listed_names: &[CARD_ZARC, 22211622], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Enable pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --Take no battle or effect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCondition(s.ndcon)
    effect::set_condition(d, e1, Some(condition(ndcon)));
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Add 1 "Polymerization" spell or "fusion" spell from deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special summon 1 dragon fusion or synchro monster from extra deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Place itself into pendulum zone
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e6, EVENT_DESTROYED);
    // e6:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    // e6:SetCondition(s.pencon)
    effect::set_condition(d, e6, Some(condition(pencon)));
    // e6:SetTarget(s.pentg)
    effect::set_target(d, e6, Some(cost_target(pentg)));
    // e6:SetOperation(s.penop)
    effect::set_operation(d, e6, Some(operation(penop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // --Reset
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e7, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e7:SetCode(EVENT_ADJUST)
    effect::set_code(d, e7, EVENT_ADJUST);
    // e7:SetRange(LOCATION_PZONE)
    effect::set_range(d, e7, LOCATION_PZONE);
    // e7:SetLabelObject(e1)
    effect::set_label_object(d, e7, Value::Effect(e1));
    // e7:SetOperation(s.trig)
    effect::set_operation(d, e7, Some(operation(trig)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
// function s.ndcfilter(c)
fn ndcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_ZARC)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ZARC])
}
// function s.ndcon(e)
fn ndcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.ndcfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&ndcfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.damval(e,re,val,r,rp,rc)
fn damval(d: &mut Duel, a: Args) -> i64 {
    // local tp=e:GetHandlerPlayer()
    let _tp = effect::get_handler_player(d, a.e(0));
    let val = a.int(2);
    // if val~=0 then
    if val != 0 {
        // e:SetLabel(val)
        effect::set_label(d, a.e(0), &[val]);
        // return 0
        return 0;
    }
    // else return val end
    val
}
// function s.trig(e,tp,eg,ep,ev,re,r,rp)
fn trig(d: &mut Duel, a: &EffArgs) {
    // local val=e:GetLabelObject():GetLabel()
    let le = effect::get_label_object(d, a.e).as_effect().nil();
    let val = effect::get_label(d, le);
    // if val~=0 then
    if val != 0 {
        // Duel.RaiseEvent(e:GetHandler(),id,e,REASON_EFFECT,tp,tp,val)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_event(d, h, ID, Some(a.e), REASON_EFFECT, a.tp, a.tp, val as u32);
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, le, &[0]);
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_PZONE,0,1,e:GetHandler(),22211622)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[22211622])), a.tp, LOCATION_PZONE, 0, 1, h.into())
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSetCard(SET_FUSION) and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_set_card(d, c, &[SET_FUSION as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // if Duel.Destroy(g,REASON_EFFECT)==2 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
// function s.desfilter(c,e,tp,mc)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,Group.FromCards(c,mc))
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(mc));
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(&mg))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,mg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mg: Option<&CardSet>) -> bool {
    // return c:IsType(TYPE_FUSION+TYPE_SYNCHRO) and c:IsRace(RACE_DRAGON) and Duel.GetLocationCountFromEx(tp,tp,mg,c)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO)
        && card::is_race(d, c, RACE_DRAGON as u64)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mg.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(tp) and s.desfilter(chkc,e,tp,c) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, tp) && desfilter(d, chkc, e, tp, c) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,c,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, e, tp, c)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,c,e,tp,c)
    let mut g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, e, tp, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc or not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local dg=Group.FromCards(c,tc)
    let mut dg = CardSet::from_card(c);
    group::add_card(&mut dg, &CardSet::from_card(tc));
    // if Duel.Destroy(dg,REASON_EFFECT)==2 then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sc=g:GetFirst()
        let sc = group::get_first(d, &g).nil();
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // --Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e2,true)
            card::register_effect(d, sc, e2, true);
            // --ATK/DEF becomes 0
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
            // e3:SetValue(0)
            effect::set_value(d, e3, 0);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e3,true)
            card::register_effect(d, sc, e3, true);
            // local e4=e3:Clone()
            let e4 = effect::clone(d, e3);
            // e4:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e4, EFFECT_SET_DEFENSE_FINAL);
            // sc:RegisterEffect(e4,true)
            card::register_effect(d, sc, e4, true);
            // --Cannot be used as synchro material
            // local e5=e3:Clone()
            let e5 = effect::clone(d, e3);
            // e5:SetDescription(3310)
            effect::set_description(d, e5, 3310);
            // e5:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e5, EFFECT_FLAG_CLIENT_HINT, 0);
            // e5:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
            effect::set_code(d, e5, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
            // e5:SetValue(1)
            effect::set_value(d, e5, 1);
            // sc:RegisterEffect(e5,true)
            card::register_effect(d, sc, e5, true);
            // --Cannot be used as Xyz material
            // local e6=e5:Clone()
            let e6 = effect::clone(d, e5);
            // e6:SetDescription(3311)
            effect::set_description(d, e6, 3311);
            // e6:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
            effect::set_code(d, e6, EFFECT_CANNOT_BE_XYZ_MATERIAL);
            // sc:RegisterEffect(e6,true)
            card::register_effect(d, sc, e6, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
// function s.pencon(e,tp,eg,ep,ev,re,r,rp)
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
// function s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    // if e:GetHandler():IsLocation(LOCATION_GRAVE) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    }
    true
}
// function s.penop(e,tp,eg,ep,ev,re,r,rp)
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
