//! The Crimson King
//! 紅蓮の王者
use super::prelude::*;

pub const ID: u32 = 67809530;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner DARK monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_DARK),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, cc, sc, st, tp| card::is_attribute_ctx(d, cc, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Add 1 card that mentions "Red Dragon Archfiend" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_synchro_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Red Dragon Archfiend" from your Extra Deck (this is treated as a Synchro Summon)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsMonsterEffect() end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let re = a.re.nil();
            a.rp == 1 - a.tp && effect::is_monster_effect(d, re)
        })),
    );
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_RED_DRAGON_ARCHFIEND) and c:IsAbleToHand()
    card::lists_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
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
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // if not c:IsCode(CARD_RED_DRAGON_ARCHFIEND) then return false end
    if !card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) {
        return false;
    }
    // local forcedg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_SYNCHRO)
    let forcedg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_SYNCHRO as u32);
    // return #forcedg==0 and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    let ex = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    forcedg.is_empty()
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, Some(h))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &sel) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP) then
    //     (a number is always true in a condition)
    duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
    // sc:CompleteProcedure()
    card::complete_procedure(d, sc);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, effect::get_handler(d, e).nil());
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(sc:GetAttack()*2)
    let atk = card::get_attack(d, sc);
    effect::set_value(d, e1, atk as i64 * 2);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(3060)
    effect::set_description(d, e2, 3060);
    // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // sc:RegisterEffect(e2)
    card::register_effect(d, sc, e2, false);
}
