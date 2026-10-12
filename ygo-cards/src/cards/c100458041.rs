//! Xenovader△ Judgment
//! 異解△審判
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100458041;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

// The script state of Judgment (`s.sumgroup`, `s.attr_list`), kept in the duel:
// 0 = `s.sumgroup` (a group), 1 + tp = `s.attr_list[tp]` (the attributes chosen this turn).
const KEY_SUMGROUP: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If your opponent Normal or Special Summons a monster(s): You can target 1 of them; choose 1 of your face-down banished "Xenovader△" monsters with the same Attribute, and either add it to your hand or Special Summon it, also for the rest of this turn, you cannot choose the same Attribute again with this effect of "Xenovader△ Judgment", and you cannot activate cards or effects, except "Xenovader△" cards or effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function() return not Duel.IsDamageStep() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| !duel::is_damage_step(d, None))));
    // e1:SetTarget(s.thsptg)
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the Normal or Special Summoned monsters
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s.sumgroup=Group.CreateGroup()
        // (`KeepAlive` is not needed: the group is a value of the duel)
        set_sumgroup(d, CardSet::new());
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetOperation(s.sumgroupregop)
        effect::set_operation(d, ge1, Some(operation(sumgroupregop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // s.attr_list={}
        // s.attr_list[0]=0
        // s.attr_list[1]=0
        set_attr_list(d, 0, 0);
        set_attr_list(d, 1, 0);
        // aux.AddValuesReset(function() s.attr_list[0]=0 s.attr_list[1]=0 end)
        aux::add_values_reset(d, reset_attr_list);
    });
    // Once per battle, during damage calculation, if your "Xenovader△" monster battles an opponent's monster and you have no cards in your Deck: You can make that opponent's monster's ATK become 0
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(Cost.SoftOncePerBattle)
    // (used without arguments: it always passes and records nothing, see c56832966.rs)
    effect::set_cost(d, e2, Some(cost_no_op()));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}

/// `Cost.SoftOncePerBattle` used without arguments (see c56832966.rs)
fn cost_no_op() -> LuaFn {
    cost_target(|_d, _a| true)
}

/// `s.sumgroup`
fn sumgroup(d: &Duel) -> CardSet {
    match s_var(d, ID, KEY_SUMGROUP) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    }
}

fn set_sumgroup(d: &mut Duel, g: CardSet) {
    set_s_var(d, ID, KEY_SUMGROUP, Value::Group(g));
}

/// `s.attr_list[tp]`
fn attr_list(d: &Duel, tp: u8) -> i64 {
    s_var(d, ID, 1 + tp as u64).int()
}

fn set_attr_list(d: &mut Duel, tp: u8, v: i64) {
    set_s_var(d, ID, 1 + tp as u64, Value::Int(v));
}

/// `function() s.attr_list[0]=0 s.attr_list[1]=0 end`
fn reset_attr_list(d: &mut Duel) {
    set_attr_list(d, 0, 0);
    set_attr_list(d, 1, 0);
}

/// `s.sumgroupregop(e,tp,eg,ep,ev,re,r,rp)`
fn sumgroupregop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(Card.IsFaceup,nil)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        //     tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
        // end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // if Duel.GetCurrentChain()==0 then s.sumgroup:Clear() end
        let mut sg = sumgroup(d);
        if duel::get_current_chain(d, false) == 0 {
            sg = CardSet::new();
        }
        // s.sumgroup:Merge(tg)
        group::add_card(&mut sg, &tg);
        // s.sumgroup:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut sg, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        set_sumgroup(d, sg.clone());
        // Duel.RaiseEvent(s.sumgroup,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_event(d, sg, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}

/// `s.tgfilter(c,e,tp,opp,mmz_chk)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, opp: u8, mmz_chk: bool) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsCanBeEffectTarget(e) and c:IsSummonPlayer(opp)
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp,mmz_chk,c:GetAttribute())
    if !(card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_summon_player(d, c, opp))
    {
        return false;
    }
    let attr = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, e, tp, mmz_chk, attr)), tp, LOCATION_REMOVED, 0, 1, NO_EX)
}

/// `s.thspfilter(c,e,tp,mmz_chk,attr)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool, attr: u32) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and c:IsAttribute(attr) and s.attr_list[tp]&c:GetAttribute()==0
    //     and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_facedown(d, c)
        && card::is_set_card(d, c, &[SET_XENOVADER as u16])
        && card::is_attribute(d, c, attr)
        && (attr_list(d, tp) as u32 & card::get_attribute(d, c)) == 0
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}

/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local g=s.sumgroup:Filter(s.tgfilter,nil,e,tp,1-tp,mmz_chk)
    let sg = sumgroup(d);
    let g = group::filter(d, &sg, &|d, c| tgfilter(d, c, a.e, tp, 1 - tp, mmz_chk), NO_EX);
    // if chkc then return g:IsContains(chkc) and g:IsExists(s.tgfilter,1,nil,e,tp,1-tp,mmz_chk) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && group::is_exists(d, &g, &|d, c| tgfilter(d, c, a.e, tp, 1 - tp, mmz_chk), 1, NO_EX);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tc=nil
    // if #g==1 then
    let tc = if g.len() == 1 {
        // tc=g:GetFirst()
        // Duel.SetTargetCard(tc)
        group::get_first(d, &g).nil()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    };
    duel::set_target_card(d, tc);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}

/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp,mmz_chk,tc:GetAttribute()):GetFirst()
        let tc_attr = card::get_attribute(d, tc);
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| thspfilter(d, x, a.e, tp, mmz_chk, tc_attr)),
            tp,
            LOCATION_REMOVED,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &sel) {
            // local attr=sc:GetAttribute()
            let attr = card::get_attribute(d, sc);
            // s.attr_list[tp]=s.attr_list[tp]|attr
            set_attr_list(d, tp, attr_list(d, tp) | attr as i64);
            // aux.ToHandOrElse(sc,tp,
            //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
            //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
            //     aux.Stringid(id,3))
            let e = a.e;
            aux::to_hand_or_else(
                d,
                sc,
                tp,
                Some(&|d, _x| mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
                Some(&|d, _g| duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
                Some(aux::stringid(ID, 3)),
            );
            // if sc:IsLocation(LOCATION_HAND) then Duel.ConfirmCards(1-tp,sc) end
            if card::is_location(d, sc, LOCATION_HAND) {
                duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
            }
            // for _,str in aux.GetAttributeStrings(attr) do
            for str in aux::get_attribute_strings(attr as u64) {
                // c:RegisterFlagEffect(0,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,str)
                card::register_flag_effect(d, c, 0, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, str);
            }
        }
    }
    // You cannot activate cards or effects for the rest of this turn, except "Xenovader△" cards or effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e,re) return not re:GetHandler():IsSetCard(SET_XENOVADER) end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d, a| {
            let rh = effect::get_handler(d, a.e(1)).nil();
            !card::is_set_card(d, rh, &[SET_XENOVADER as u16])
        }),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}

/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 then return false end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 {
        return false;
    }
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, tp);
    // return bc1 and bc1:IsSetCard(SET_XENOVADER) and bc2 and bc2:IsControler(1-tp) and bc2:HasNonZeroAttack()
    match (bc1, bc2) {
        (Some(bc1), Some(bc2)) => card::is_set_card(d, bc1, &[SET_XENOVADER as u16]) && card::is_controler(d, bc2, 1 - tp) && card::has_non_zero_attack(d, bc2),
        _ => false,
    }
}

/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local _,bc=Duel.GetBattleMonster(tp)
    let (_, bc) = duel::get_battle_monster(d, a.tp);
    let bc = bc.nil();
    // if bc:IsRelateToBattle() and bc:IsFaceup() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // Make that opponent's monster's ATK become 0
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
