//! Cyberse Code Magician
use super::prelude::*;

pub const ID: u32 = 64865;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[34767865],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // If a Link Monster you control would be used as Link Material for a Cyberse monster, this card in your hand can also be used as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetOperation(s.extracon)
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    // e1:SetValue(s.extraval)
    effect::set_value_fn(d, e1, lua_fn(extraval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is sent from the hand or field to the GY: You can send 1 Cyberse monster from your Deck to the GY, ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_HAND | LOCATION_ONFIELD)
    })));
    // e2:SetTarget(s.tgsptg)
    effect::set_target(d, e2, Some(cost_target(tgsptg)));
    // e2:SetOperation(s.tgspop)
    effect::set_operation(d, e2, Some(operation(tgspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The Lua table `s.flagmap1[c]` (the flag effects this card registered as material), kept in the duel.
fn flag_key(c: CardId) -> u64 {
    0x20000 + c.0 as u64
}
fn flag_list(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flag_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flag_list(d: &mut Duel, c: CardId, list: Vec<EffectId>) {
    set_s_var(d, ID, flag_key(c), Value::Multi(list.into_iter().map(Value::Effect).collect()));
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // return (sg+mg):Filter(s.extrafilter,nil,e:GetHandlerPlayer()):IsExists(Card.IsLinkMonster,1,og) and
    //     sg:FilterCount(Card.HasFlagEffect,nil,id)<2
    let mut union = sg.clone();
    group::add_card(&mut union, &mg);
    let hp = effect::get_handler_player(d, e);
    let filtered = group::filter(d, &union, &|d, c| extrafilter(d, c, hp), NO_EX);
    let ex: Exclude = (&og).into();
    let exists = group::is_exists(d, &filtered, &|d, c| card::is_link_monster(d, c), 1, ex);
    exists && group::filter_count(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX) < 2
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.c(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsRace(RACE_CYBERSE) or Duel.GetFlagEffect(tp,id)>0 then
        //     return Group.CreateGroup()
        if summon_type != SUMMON_TYPE_LINK as i64
            || !card::is_race(d, sc, RACE_CYBERSE as u64)
            || duel::get_flag_effect(d, tp, ID) > 0
        {
            return Value::Group(group::create_group(&[]));
        }
        // table.insert(s.flagmap1[c],c:RegisterFlagEffect(id,0,0,1))
        let eff = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
        let mut list = flag_list(d, c);
        list.push(eff);
        set_flag_list(d, c, list);
        // return Group.FromCards(c)
        Value::Group(CardSet::from_card(c))
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK == SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 1);
        }
        Value::Nil
    } else if chk == 2 {
        // for _,eff in ipairs(s.flagmap1[c]) do eff:Reset() end
        for eff in flag_list(d, c) {
            effect::reset(d, eff);
        }
        // s.flagmap1[c]={}
        set_flag_list(d, c, Vec::new());
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.tgspfilter(c,e,tp,mmz_ritsum_chk)`
fn tgspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_ritsum_chk: bool) -> bool {
    // return c:IsRace(RACE_CYBERSE) and (c:IsAbleToGrave() or (mmz_ritsum_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_race(d, c, RACE_CYBERSE as u64)
        && (card::is_able_to_grave(d, c, None)
            || (mmz_ritsum_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.tgsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ritual_summoned_chk=e:GetHandler():IsRitualSummoned()
    let h = effect::get_handler(d, e).nil();
    let ritual_summoned_chk = card::is_ritual_summoned(d, h);
    // if chk==0 then
    if a.chk == Some(0) {
        // local mmz_ritsum_chk=ritual_summoned_chk and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_ritsum_chk = ritual_summoned_chk && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.tgspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,mmz_ritsum_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| tgspfilter(d, c, e, tp, mmz_ritsum_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // if not ritual_summoned_chk then
    if !ritual_summoned_chk {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, e, CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else {
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
        // e:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.tgspop(e,tp,eg,ep,ev,re,r,rp)`
fn tgspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local mmz_ritsum_chk=e:GetLabel()==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_ritsum_chk = effect::get_label(d, e) == 1 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local hint=mmz_ritsum_chk and aux.Stringid(id,1) or HINTMSG_TOGRAVE
    let hint = if mmz_ritsum_chk { aux::stringid(ID, 1) } else { HINTMSG_TOGRAVE as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hint)
    duel::hint(d, HINT_SELECTMSG as u8, tp, hint);
    // local sc=Duel.SelectMatchingCard(tp,s.tgspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_ritsum_chk):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| tgspfilter(d, c, e, tp, mmz_ritsum_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // local op=nil
        // if not mmz_ritsum_chk then op=1
        // else
        //     local b1=sc:IsAbleToGrave()
        //     local b2=sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
        // end
        let op: Option<usize> = if !mmz_ritsum_chk {
            Some(1)
        } else {
            let b1 = card::is_able_to_grave(d, sc, None);
            let b2 = card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None);
            duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
        };
        // if op==1 then Duel.SendtoGrave(sc,REASON_EFFECT)
        if op == Some(1) {
            duel::send_to_grave(d, CardSet::from_card(sc), REASON_EFFECT, None, None);
        // elseif op==2 then Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        } else if op == Some(2) {
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // You cannot Special Summon for the rest of this turn, except Cyberse monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsRaceExcept(RACE_CYBERSE) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_CYBERSE as u64, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
