//! Deep-Eyes White Dragon, the Blue Abyss
use super::prelude::*;

pub const ID: u32 = 67886895;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON], listed_series: &[SET_BLUE_EYES], ..ScriptInfo::DEFAULT };

/// `s.gygroup`: the "Blue-Eyes" Ritual Monsters and "Blue-Eyes White Dragon" sent to the GY (in the duel)
const GYGROUP: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can discard this card; add 1 Level 1 LIGHT Tuner from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);

    // If a "Blue-Eyes" Ritual Monster(s) and/or "Blue-Eyes White Dragon" is sent to your GY: You can target 1 of them; Special Summon this card from your GY (if it was there when the monster was sent) or hand (even if not), and if you do, its ATK becomes that monster's, also banish it when it leaves the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_range(d, e2, LOCATION_GRAVE | LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);

    // Keep track of "Blue-Eyes" Ritual Monsters and "Blue-Eyes White Dragon" sent to the GY
    aux::global_check(d, ID, |d| {
        // s.gygroup=Group.CreateGroup()
        set_gygroup(d, group::create_group(&[]));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        effect::set_operation(d, ge1, Some(operation(regop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
fn gygroup(d: &Duel) -> CardSet {
    s_var(d, ID, GYGROUP).as_group().cloned().nil()
}
fn set_gygroup(d: &mut Duel, g: CardSet) {
    set_s_var(d, ID, GYGROUP, Value::Group(g));
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(1) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsType(TYPE_TUNER) and c:IsAbleToHand()
    card::is_level(d, c, &[1]) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return ((c:IsSetCard(SET_BLUE_EYES) and c:IsRitualMonster()) or c:IsCode(CARD_BLUEEYES_W_DRAGON))
    //     and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_GRAVE)
    ((card::is_set_card(d, c, &[SET_BLUE_EYES as u16]) && card::is_ritual_monster(d, c)) || card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_GRAVE)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tg=eg:Filter(s.tgfilter,nil,e)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| tgfilter(d, c, e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // if Duel.GetCurrentChain()==0 then s.gygroup:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            let mut gg = gygroup(d);
            gg.clear();
            set_gygroup(d, gg);
        }
        // s.gygroup:Merge(tg)
        let mut gg = gygroup(d);
        group::add_card(&mut gg, &tg);
        set_gygroup(d, gg);
        // s.gygroup:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        let mut gg = gygroup(d);
        group::remove(d, &mut gg, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        set_gygroup(d, gg.clone());
        // Duel.RaiseEvent(s.gygroup,EVENT_CUSTOM+id,e,0,rp,ep,ev)
        duel::raise_event(d, gg, EVENT_CUSTOM + ID, Some(e), 0, a.rp, a.ep, a.ev);
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local g=s.gygroup:Filter(s.tgfilter,nil,e):Match(Card.IsControler,nil,tp)
    let gg = gygroup(d);
    let mut g = group::filter(d, &gg, &|d, c| tgfilter(d, c, e), NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and chkc:IsControler(tp) and s.tgfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, e);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // local tc=nil
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // tc=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, e) {
            // Its ATK becomes that monster's
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, atk as i64);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        // Banish it when it leaves the field
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, 3300);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_value(d, e2, LOCATION_REMOVED as i64);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e2,true)
        card::register_effect(d, c, e2, true);
    }
}
