//! Speedroid Wing Synchron
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 2254222;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[82044279], listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Destroy both this card and 1 Level 2 WIND Pendulum Monster Card in your Pendulum Zone, and if you do, Special Summon 1 "Clear Wing Synchro Dragon"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 "Speedroid" Spell/Trap from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(lua_fn(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(2) and c:IsAttribute(ATTRIBUTE_WIND)
    card::is_level(d, c, &[2]) && card::is_attribute(d, c, ATTRIBUTE_WIND)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(82044279) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    card::is_code(d, c, &[82044279])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_PZONE,0,1,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_PZONE, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local pc=Duel.GetMatchingGroup(nil,tp,LOCATION_PZONE,0,c):GetFirst()
    let pg = duel::get_matching_group(d, None, tp, LOCATION_PZONE, 0, c.into());
    let pc = group::get_first(d, &pg);
    // if c:IsRelateToEffect(e) and pc and s.desfilter(pc) and Duel.Destroy(Group.FromCards(c,pc),REASON_EFFECT)==2 then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(pc) = pc {
            if desfilter(d, pc) {
                let mut both = CardSet::from_card(c);
                group::add_card(&mut both, &CardSet::from_card(pc));
                if duel::destroy(d, both, REASON_EFFECT, None, None) == 2 {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
                    let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
                    // if sc and Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
                    if let Some(sc) = group::get_first(d, &sg) {
                        if duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                            // sc:CompleteProcedure()
                            card::complete_procedure(d, sc);
                        }
                    }
                }
            }
        }
    }
    // Cards in your Pendulum Zone cannot be destroyed by card effects for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_PZONE,0)
    effect::set_target_range(d, e1, LOCATION_PZONE, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // You cannot Special Summon, except WIND monsters, for the rest of this turn
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e2, aux::stringid(ID, 3));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(e,c) return not c:IsAttribute(ATTRIBUTE_WIND) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_attribute(d, c, ATTRIBUTE_WIND))));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)
}
/// `s.windsynchrofilter(c)`
fn windsynchrofilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsSpellTrap() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.windsynchrofilter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&windsynchrofilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) and g:GetBinClassCount(Card.GetRace)>=2
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && group::get_bin_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
